package io.bosca.workflow.search

import com.meilisearch.sdk.*
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.StorageSystemType
import io.bosca.util.decode
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext

class DeleteFromIndexes(client: io.bosca.api.Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Delete Item from Search Indexes",
            description = "Delete Item from Search Indexes",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val metadata = job.metadata?.metadata ?: error("metadata missing")
        val meilisearchConfig = client.configurations.get<Configuration>("meilisearch") ?: error("meilisearch configuration missing")
        val client = Client(meilisearchConfig.toConfig())
        val taskIds = mutableListOf<Pair<Index, Int>>()
        for (system in job.storageSystems.filter { it.system.storageSystem.type == StorageSystemType.SEARCH }) {
            val cfg = system.system.storageSystem.configuration.decode<IndexConfiguration>() ?: error("index configuration missing")
            val index = client.index(cfg.name)
            val id = index.deleteDocument(metadata.id).taskUid
            taskIds.add(index to id)
        }
        for ((index, taskId) in taskIds) {
            index.suspendWaitForTask(taskId)
        }
    }

    companion object {
        const val ID = "metadata.search.delete.item"
    }
}