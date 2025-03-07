package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext

class DeleteSupplementary(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Delete Supplementary Metadata",
            description = "Delete a Supplementary Metadata",
            inputs = listOf(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val key = getInputParameterValue(job, INPUT_NAME)
        client.metadata.deleteSupplementary(
            job.metadata?.metadata?.id ?: error("metadata id missing"),
            key
        )
    }

    companion object {
        const val ID = "metadata.supplementary.delete"
        const val INPUT_NAME = "supplementary"
    }
}