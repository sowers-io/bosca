package io.bosca.workflow.collection

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
            name = "Run Delayed Collection Transition",
            description = "Run Delayed a Collection Transition",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        if (job.collection!!.collection.collectionWorkflow.collectionWorkflow.pending == null) return
        val delayUntil = job.collection!!.collection.collectionWorkflow.collectionWorkflow.stateValid
        if (delayUntil != null && delayUntil.isAfter(ZonedDateTime.now())) {
            println("ERROR: Should delay until $delayUntil")
            throw DelayedUntilException(delayUntil)
        }
        client.workflows.beginCollectionTransition(
            job.collection?.collection?.id ?: error("missing collection"),
            job.collection!!.collection.collectionWorkflow.collectionWorkflow.pending!!,
            "Restart Collection Transition",
            true
        )
    }

    companion object {
        const val ID = "collection.restart.transition.to"
    }
}