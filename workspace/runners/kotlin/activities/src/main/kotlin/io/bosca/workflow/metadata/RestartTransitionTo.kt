package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import io.bosca.workflow.DelayedUntilException
import java.time.ZonedDateTime

class RestartTransitionTo(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Run Delayed Metadata Transition",
            description = "Run Delayed a Metadata Transition",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        if (job.metadata!!.metadata.workflow.metadataWorkflow.pending == null) return
        val delayUntil = job.metadata!!.metadata.workflow.metadataWorkflow.stateValid
        if (delayUntil != null && delayUntil.isAfter(ZonedDateTime.now())) {
            println("ERROR: Should delay until $delayUntil")
            throw DelayedUntilException(delayUntil)
        }
        client.workflows.beginMetadataTransition(
            job.metadata?.metadata?.id ?: error("missing metadata"),
            job.metadata!!.metadata.version,
            job.metadata!!.metadata.workflow.metadataWorkflow.pending!!,
            "Restart Metadata Transition",
            true
        )
    }

    companion object {
        const val ID = "metadata.restart.transition.to"
    }
}