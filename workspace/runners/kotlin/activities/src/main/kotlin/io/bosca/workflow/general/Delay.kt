package io.bosca.workflow.general

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.*
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import io.bosca.workflow.DelayedUntilException
import kotlinx.serialization.Serializable
import java.time.ZonedDateTime

@Serializable
data class DelayedContext(val delayed: Boolean = false)

@Serializable
data class DelayConfiguration(val delayUntil: String? = null, val delayFor: Int? = null)

class Delay(client: Client) : Activity(client) {
    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Delay the Workflow",
            description = "Delay the workflow for a given amount of time",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val ctx = getContext<DelayedContext>(job)
        if (ctx.delayed) return
        setContext(job, DelayedContext(delayed = true))
        val cfg = getConfiguration<DelayConfiguration>(job)
        cfg.delayUntil?.let {
            throw DelayedUntilException(ZonedDateTime.parse(it))
        }
        cfg.delayFor?.let {
            throw DelayedUntilException(ZonedDateTime.now().plusSeconds(it.toLong()))
        }
    }

    companion object {

        const val ID = "workflow.general.delay"
    }
}