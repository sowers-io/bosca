package io.bosca.workflow.collection

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
            name = "Delete All Collection Supplementary",
            description = "Delete All Collection Supplementary",
            inputs = listOf(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val supplementary = job.collection?.collection?.supplementary?.map {
            it.collectionSupplementary.id
        }
        for (id in supplementary ?: emptyList()) {
            client.collections.deleteSupplementary(id)
        }
    }

    companion object {
        const val ID = "collection.supplementary.delete.all"
    }
}