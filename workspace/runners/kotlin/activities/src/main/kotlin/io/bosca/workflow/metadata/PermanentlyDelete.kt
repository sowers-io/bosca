package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext

class PermanentlyDelete(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Permanently Delete Metadata",
            description = "Permanently Delete Metadata",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        client.metadata.deletePermanently(job.metadata?.metadata?.id ?: error("metadata id missing"))
    }

    companion object {
        const val ID = "metadata.delete.permanently"
    }
}