package io.bosca.workflow

import com.apollographql.apollo.exception.ApolloNetworkException
import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import kotlinx.coroutines.*
import kotlinx.coroutines.channels.ReceiveChannel
import kotlinx.coroutines.channels.produce
import java.util.concurrent.Executors
import java.util.concurrent.atomic.AtomicLong

class JobRunner(
    private val client: Client,
    private val queue: String,
    private val max: Int,
    private val registry: ActivityRegistry,
) {

    private val scope = CoroutineScope(SupervisorJob() + dispatcher)
    private val active = AtomicLong(0)
    private val jobs = AtomicLong(0)

    fun isShutdown(): Boolean = !scope.isActive

    fun shutdown() {
        scope.cancel()
    }

    @OptIn(ExperimentalCoroutinesApi::class)
    private fun CoroutineScope.getNextJob() = produce(capacity = max) {
        var delay = 1L
        val pending = mutableListOf<Deferred<WorkflowJob?>>()
        val max = Runtime.getRuntime().availableProcessors() * 2
        var current = 1
        try {
            while (isActive) {
                try {
                    while (pending.size < current) {
                        pending += scope.async {
                            try {
                                client.workflows.getNextJob(queue)
                            } catch (e: ApolloNetworkException) {
                                println("error fetching next job: $queue: $e :: ${e.platformCause}")
                                null
                            }
                        }
                    }
                    var found = false
                    for (job in pending) {
                        try {
                            val result = job.await()
                            if (result != null) {
                                found = true
                                send(result)
                            }
                        } catch (e: CancellationException) {
                            throw e
                        } catch (e: Exception) {
                            println("error fetching next job: $queue: $e")
                            continue
                        }
                    }
                    pending.clear()
                    if (!found) {
                        delay(delay)
                        delay = minOf(delay * 2, 3_000)
                        current = 1
                    } else {
                        delay = 1L
                        current = minOf(current + 1, max)
                    }
                } catch (e: CancellationException) {
                    break
                } catch (e: Exception) {
                    println("error fetching next job: $queue: $e")
                    delay(1_000)
                    continue
                }
            }
        } catch (ignore: CancellationException) {
        }
    }

    fun run() {
        val producer = scope.getNextJob()
        repeat(max) {
            scope.launch {
                producer.process()
            }
        }
    }

    private suspend fun CoroutineScope.checkin(id: WorkflowJob.Id) {
        while (isActive) {
            try {
                client.workflows.setWorkflowJobCheckin(id)
            } catch (e: CancellationException) {
                println("cancelled checkin: $id")
                break
            } catch (e: Exception) {
                if (e.message?.contains("can't update plan, it's already finished") != true) {
                    println("failed to checkin: ${id}: $e")
                }
            }
            try {
                delay(60_000)
            } catch (e: CancellationException) {
                break
            } catch (e: Exception) {
                println("failed to checkin delay: ${id}: $e")
            }
        }
    }

    @OptIn(DelicateCoroutinesApi::class)
    private suspend fun ReceiveChannel<WorkflowJob>.process() {
        for (job in this) {
            if (isClosedForReceive) {
                break
            }
            val checkin = scope.launch { checkin(job.id) }
            try {
                job.execute()
            } finally {
                checkin.cancel()
            }
        }
    }

    private suspend fun WorkflowJob.execute() {
        try {
            active.incrementAndGet()
            val activity = registry.getActivity(workflowActivity.workflowActivity.activityId)
            if (activity == null) {
                println("missing activity: ${workflowActivity.workflowActivity.activityId}")
                client.workflows.setWorkflowJobFailed(
                    id,
                    "missing activity: ${workflowActivity.workflowActivity.activityId}"
                )
                return
            }
            println("processing job: $id : ${workflowActivity.workflowActivity.activityId} : ${jobs.incrementAndGet()} : ${active.get()}")
            val context = ActivityContext()
            try {
                activity.execute(context, this)
            } finally {
                context.cleanup()
            }
            client.workflows.setWorkflowJobComplete(id)
            println("processing complete: ${id} : ${workflowActivity.workflowActivity.activityId} : ${active.get()}")
        } catch (e: CancellationException) {
            println("cancelled job")
        } catch (e: DelayedUntilException) {
            client.workflows.setWorkflowJobDelayedUntil(id, e.delayedUntil)
            println("delayed executing: ${id}: $e")
        } catch (e: Exception) {
            client.workflows.setWorkflowJobFailed(id, e.toString())
            println("failed to execute: ${id}: $e")
            e.printStackTrace()
        } finally {
            active.decrementAndGet()
        }
    }

    companion object {
        val dispatcher = Executors.newCachedThreadPool().asCoroutineDispatcher()
    }
}
