package io.bosca.workflow.collection

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.util.toOptional
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext

class BeginTransitionTo(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Begin Collection Transition",
            description = "Begin a Collection Transition",
            configuration = mapOf<String, Any>(
                "state" to "draft",
            ).toOptional(),
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val configuration = job.workflowActivity.workflowActivity.configuration as Map<*, *>
        val state = configuration["state"] as String
        val current = job.collection?.collection?.workflow?.collectionWorkflow
        if (state != current?.state && state != current?.pending) {
            client.workflows.beginCollectionTransition(
                job.collection?.collection?.id ?: error("missing collection"),
                "Begin Collection Transition",
                state
            )
        }
    }

    companion object {
        const val ID = "collection.begin.transition.to"
    }
}