package io.bosca.workflow.collection

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.util.toOptional
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext

class TransitionTo(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Finalize Collection Transition",
            description = "Finalize a Collection Transition",
            configuration = mapOf<String, Any>(
                "state" to "draft",
                "status" to "marked draft",
            ).toOptional(),
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val collection = job.collection?.collection ?: return
        val configuration = job.workflowActivity.workflowActivity.configuration as Map<*, *>
        val state = configuration["state"] as String
        val status = if (configuration.containsKey("status")) configuration["status"] as String else ""
        client.workflows.setCollectionWorkflowStateComplete(collection.id, status)
        client.workflows.setCollectionWorkflowState(collection.id, state, status, true)
    }

    companion object {
        const val ID = "collection.transition.to"
    }
}