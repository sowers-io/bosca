package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext

class DeleteAllPlanSupplementary(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Delete All Plan Supplementary Metadata",
            description = "Delete All Plan Supplementary Metadata",
            inputs = listOf(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        job.metadata?.metadata?.supplementary?.filter { it.metadataSupplementary.planId == job.planId.id }?.forEach { supplementary ->
            client.metadata.deleteSupplementary(supplementary.metadataSupplementary.id)
        }
    }

    companion object {
        const val ID = "metadata.supplementary.delete.plan.all"
    }
}