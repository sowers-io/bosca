package io.bosca.workflow.collection

import io.bosca.api.Client
import io.bosca.api.CollectionItem
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.util.toOptional
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.coroutines.Job
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.joinAll

import kotlinx.coroutines.launch

class SetReady(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Set Collection Ready",
            description = "Set Collection Ready",
            configuration = mapOf<String, Any>(
                "public" to false,
                "recursive" to false,
                "collectionId" to ""
            ).toOptional(),
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val configuration = job.workflowActivity.workflowActivity.configuration as Map<*, *>
        val public = configuration["public"] as Boolean
        val recursive = configuration["recursive"] as Boolean
        val collectionId = configuration["collectionId"]?.toString()?.takeIf { it.isNotBlank() }
            ?: job.collection?.collection?.id
            ?: error("collection id missing")
        val collection = client.collections.get(collectionId) ?: error("collection not found")
        if (collection.ready != null) return
        setReady(
            collectionId,
            public,
            recursive
        )
    }

    private suspend fun setReady(id: String, public: Boolean, recursive: Boolean) {
        if (recursive) {
            val items = client.collections.list(id) ?: error("missing items")
            val jobs = mutableListOf<Job>()
            for (item in items.items) {
                if (item.isCollection) {
                    val collection = (item as CollectionItem.Collection).collection
                    if (collection.ready == null) {
                        jobs.add(coroutineScope {
                            launch {
                                setReady(item.id, public, true)
                            }
                        })
                    }
                } else {
                    val metadata = (item as CollectionItem.Metadata).metadata
                    if (metadata.ready == null) {
                        jobs.add(coroutineScope {
                            launch {
                                client.metadata.setReady(item.id)
                            }
                        })
                    }
                }
            }
            jobs.joinAll()
        }
        if (public) {
            client.collections.setPublic(id, true)
        }
        client.collections.setReady(id)
    }

    companion object {
        const val ID = "collection.set.ready"

        fun newConfiguration(recursive: Boolean, public: Boolean, collectionId: String): Map<String, Any> = mapOf(
            "recursive" to recursive,
            "public" to public,
            "collectionId" to collectionId,
        )
    }
}