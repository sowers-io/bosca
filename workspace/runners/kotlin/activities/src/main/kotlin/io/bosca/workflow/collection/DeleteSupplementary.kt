package io.bosca.workflow.collection

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.ActivityParameterInput
import io.bosca.graphql.type.ActivityParameterType
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext

class DeleteSupplementary(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Delete Collection Supplementary",
            description = "Delete a Collection Supplementary",
            inputs = listOf(
                ActivityParameterInput(
                    INPUT_NAME,
                    ActivityParameterType.SUPPLEMENTARY
                )
            ),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        deleteCollectionSupplementary(job, INPUT_NAME)
    }

    companion object {
        const val ID = "collection.supplementary.delete"
        const val INPUT_NAME = "supplementary"
    }
}