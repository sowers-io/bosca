package io.bosca.workflow.collection

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
            name = "Set Collection Public",
            description = "Set Collection Public",
            configuration = mapOf<String, Any>(
                "public" to true,
                "recursive" to true
            ).toOptional(),
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val configuration = job.workflowActivity.workflowActivity.configuration as Map<*, *>
        val public = configuration["public"] as Boolean
        val recursive = configuration["recursive"] as Boolean
        setPublic(job.collection?.collection?.id ?: error("collection id missing"), public, recursive)
    }

    private suspend fun setPublic(id: String, public: Boolean, recursive: Boolean) {
        if (recursive) {
            val items = client.collections.list(id) ?: error("missing items")
            for (item in items.items) {
                if (item.isCollection) {
                    setPublic(item.id, public, true)
                } else {
                    client.metadata.setPublic(item.id, public)
                }
            }
        }
        client.collections.setPublic(id, public)
    }

    companion object {
        const val ID = "collection.set.public"
    }
}