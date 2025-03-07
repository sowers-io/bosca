package io.bosca.workflow.collection

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext

class Traits(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Process Collection Traits",
            description = "",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val metadata = job.collection?.collection ?: error("missing collection")
        val executed = mutableSetOf<String>()
        if (job.context != null) {
            @Suppress("UNCHECKED_CAST")
            executed.addAll(((job.context as Map<*, *>)["executed"] as List<String>))
        }
        for (traitId in metadata.traitIds) {
            if (executed.contains(traitId)) continue
            val trait = client.workflows.getTrait(traitId) ?: continue
            client.workflows.enqueueChildWorkflows(
                trait.workflowIds,
                job.id
            )
            executed.add(traitId)
            client.workflows.setWorkflowJobContext(job.id, mapOf("executed" to executed.toList()))
        }
    }

    companion object {
        const val ID = "collection.traits.process"
    }
}