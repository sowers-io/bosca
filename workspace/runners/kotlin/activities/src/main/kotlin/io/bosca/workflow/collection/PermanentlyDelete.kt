package io.bosca.workflow.collection

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
            name = "Permanently Delete Collection",
            description = "Permanently Delete Collection",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        client.collections.deletePermanently(job.collection?.collection?.id ?: error("collection id missing"))
    }

    companion object {
        const val ID = "collection.delete.permanently"
    }
}