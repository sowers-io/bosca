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
        job.metadata?.metadata?.supplementary?.forEach { supplementary ->
            client.metadata.deleteSupplementary(supplementary.metadataSupplementary.id)
        }
    }

    companion object {
        const val ID = "metadata.supplementary.delete.all"
    }
}