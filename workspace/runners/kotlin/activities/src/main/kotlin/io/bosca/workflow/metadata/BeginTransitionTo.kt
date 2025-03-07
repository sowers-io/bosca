package io.bosca.workflow.metadata

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
            name = "Begin Metadata Transition",
            description = "Begin a Metadata Transition",
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

        client.workflows.beginMetadataTransition(
            job.metadata?.metadata?.id ?: error("missing metadata"),
            job.metadata!!.metadata.version,
            "Begin Metadata Transition",
            state
        )
    }

    companion object {
        const val ID = "metadata.begin.transition.to"
    }
}