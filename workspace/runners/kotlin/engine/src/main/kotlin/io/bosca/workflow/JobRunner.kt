package io.bosca.workflow

import com.apollographql.apollo.exception.ApolloNetworkException
import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import kotlinx.coroutines.*
import kotlinx.coroutines.channels.ReceiveChannel
import kotlinx.coroutines.channels.produce
import kotlinx.coroutines.flow.channelFlow
import kotlinx.coroutines.flow.toList
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

    private fun getJobs(total: Int) = channelFlow {
        val pending = mutableListOf<Job>()
        repeat(total) {
            pending += launch {
                try {
                    client.workflows.getNextJob(queue)?.let {
                        send(it)
                    }
                } catch (e: ApolloNetworkException) {
                    println("error fetching next job: $queue: $e :: ${e.platformCause}")
                }
            }
        }
        pending.joinAll()
    }

    @OptIn(ExperimentalCoroutinesApi::class)
    private fun newProducer(): ReceiveChannel<WorkflowJob> = scope.produce(capacity = max) {
        var delay = 1L
        val max = Runtime.getRuntime().availableProcessors() * 2
        var current = 1
        while (isActive) {
            try {
                val flow = getJobs(current)
                val jobs = flow.toList()
                if (jobs.isNotEmpty()) {
                    for (job in jobs) {
                        try {
                            send(job)
                        } catch (_: CancellationException) {
                            throw CancellationException()
                        } catch (e: Exception) {
                            println("error fetching next job: $queue: $e")
                            continue
                        }
                    }
                    delay = 1L
                    current = minOf(current + 1, max)
                } else {
                    delay(delay)
                    delay = minOf(delay * 2, 1_000)
                    current = 1
                }
            } catch (_: CancellationException) {
                break
            } catch (e: Exception) {
                println("error fetching next job: $queue: $e")
                delay(1_000)
            }
        }
    }

    fun run() {
        val producer = newProducer()
        repeat(max) {
            scope.launch { process(producer) }
        }
    }

    private fun checkin(id: WorkflowJob.Id) = scope.launch {
        while (isActive) {
            try {
                client.workflows.setWorkflowJobCheckin(id)
            } catch (_: CancellationException) {
                println("cancelled checkin: $id")
                break
            } catch (e: Exception) {
                if (e.message?.contains("can't update plan, it's already finished") != true) {
                    println("failed to checkin: ${id}: $e")
                }
            }
            try {
                delay(60_000)
            } catch (_: CancellationException) {
                break
            } catch (e: Exception) {
                println("failed to checkin delay: ${id}: $e")
            }
        }
    }

    @OptIn(DelicateCoroutinesApi::class)
    private suspend fun process(channel: ReceiveChannel<WorkflowJob>) {
        for (job in channel) {
            if (channel.isClosedForReceive) {
                break
            }
            val checkin = checkin(job.id)
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
                    "missing activity: ${workflowActivity.workflowActivity.activityId}",
                    true
                )
                return
            }
//            println("processing job: $id : ${workflowActivity.workflowActivity.activityId} : ${jobs.incrementAndGet()} : ${active.get()}")
            val context = ActivityContext()
            try {
                activity.execute(context, this)
            } finally {
                context.cleanup()
            }
            client.workflows.setWorkflowJobComplete(id)
//            println("processing complete: $id : ${workflowActivity.workflowActivity.activityId} : ${active.get()}")
        } catch (_: CancellationException) {
            println("cancelled job: $id")
            client.workflows.setWorkflowJobFailed(id, "job cancellation", true)
        } catch (e: DelayedUntilException) {
            println("delayed executing: ${id}: $e")
            client.workflows.setWorkflowJobDelayedUntil(id, e.delayedUntil)
        } catch (e: FullFailureException) {
            println("full failure executing: ${id}: $e")
            client.workflows.setWorkflowJobFailed(id, e.toString(), false)
        } catch (e: Exception) {
            println("failed to execute: ${id}: $e")
            client.workflows.setWorkflowJobFailed(id, e.toString(), true)
            e.printStackTrace()
        } finally {
            active.decrementAndGet()
        }
    }

    companion object {
        val dispatcher = Executors.newCachedThreadPool().asCoroutineDispatcher()
    }
}
