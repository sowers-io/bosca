package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.util.toOptional
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext

class SetPublic(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Set Metadata Public",
            description = "Set Metadata Public",
            configuration = mapOf<String, Any>(
                "public" to true
            ).toOptional(),
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val public = (job.workflowActivity.workflowActivity.configuration as Map<*, *>)["public"] as Boolean
        client.metadata.setPublic(job.metadata?.metadata?.id ?: error("metadata id missing"), public)
    }

    companion object {
        const val ID = "metadata.set.public"
    }
}