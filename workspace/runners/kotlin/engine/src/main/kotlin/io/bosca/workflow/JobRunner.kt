package io.bosca.workflow

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import kotlinx.coroutines.*
import java.util.concurrent.atomic.AtomicBoolean
import java.util.concurrent.atomic.AtomicLong
import kotlin.math.min

class JobRunner(
    private val client: Client,
    private val queue: String,
    private val max: Int,
    private val registry: ActivityRegistry,
) {

    private val shutdown = AtomicBoolean(false)
    private val active = AtomicLong(0)
    private val jobs = AtomicLong(0)
    private var noJobs = AtomicBoolean(false)
    private var delay = 1L

    fun isShutdown() = shutdown.get() && active.get() == 0L

    fun shutdown() {
        shutdown.set(true)
    }

    suspend fun run() = coroutineScope {
        while (!shutdown.get()) {
            if (active.get() >= max) {
                delay(10)
                continue
            }
            active.incrementAndGet()

            val job: WorkflowJob?
            try {
                job = client.workflows.getNextJob(queue)
                if (job == null) {
                    if (!noJobs.get()) {
                        noJobs.set(true)
                        println("no jobs available: $queue")
                    }
                    active.decrementAndGet()
                    delay = min(delay * 2, 1_000);
                    delay(delay)
                    continue
                } else if (noJobs.get()) {
                    noJobs.set(false)
                }
                delay = 1L
            } catch (e: Exception) {
                println("error fetching next job: $queue: $e")
                delay(1_000)
                active.decrementAndGet()
                continue
            }

            launch {
                try {
                    run(job)
                } catch (e: Exception) {
                    println("failed to run job: $queue: ${job.id}: $e")
                } finally {
                    active.decrementAndGet()
                }
            }
        }
    }

    private suspend fun run(job: WorkflowJob) = coroutineScope {
        val running = AtomicBoolean(true)
        val checkin = launch {
            while (running.get()) {
                try {
                    client.workflows.setWorkflowJobCheckin(job.id)
                } catch (e: CancellationException) {
                    println("cancelled checkin: ${job.id}")
                } catch (e: Exception) {
                    if (e.message?.contains("can't update plan, it's already finished") != true) {
                        println("failed to checkin: ${job.id}: $e")
                    }
                }
                if (isActive) {
                    delay(60_000)
                }
            }
        }
        try {
            val activity = registry.getActivity(job.workflowActivity.workflowActivity.activityId)
            if (activity == null) {
                println("missing activity: ${job.workflowActivity.workflowActivity.activityId}")
                client.workflows.setWorkflowJobFailed(
                    job.id,
                    "missing activity: ${job.workflowActivity.workflowActivity.activityId}"
                )
                return@coroutineScope
            }
            println("processing job: ${job.id} : ${job.workflowActivity.workflowActivity.activityId} : ${jobs.incrementAndGet()} : ${active.get()}")
            val context = ActivityContext()
            try {
                activity.execute(context, job)
            } finally {
                context.cleanup()
            }
            client.workflows.setWorkflowJobComplete(job.id)
            println("processing complete: ${job.id} : ${job.workflowActivity.workflowActivity.activityId} : ${active.get()}")
        } catch (e: DelayedUntilException) {
            client.workflows.setWorkflowJobDelayedUntil(job.id, e.delayedUntil)
            println("delayed executing: ${job.id}: $e")
        } catch (e: Exception) {
            client.workflows.setWorkflowJobFailed(job.id, e.toString())
            println("failed to execute: ${job.id}: $e")
            e.printStackTrace()
        } finally {
            running.set(false)
            checkin.cancel()
        }
    }
}