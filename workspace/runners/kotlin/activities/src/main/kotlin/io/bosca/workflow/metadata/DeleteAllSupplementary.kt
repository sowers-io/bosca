package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext

class DeleteAllSupplementary(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Delete All Supplementary Metadata",
            description = "Delete All Supplementary Metadata",
            inputs = listOf(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val workflowId = job.metadata?.metadata?.id ?: error("metadata id missing")
        for (input in job.workflowActivity.workflowActivity.inputs) {
            client.metadata.deleteSupplementary(
                workflowId,
                input.workflowActivityParameter.value
            )
        }
    }

    companion object {
        const val ID = "metadata.supplementary.delete.all"
    }
}