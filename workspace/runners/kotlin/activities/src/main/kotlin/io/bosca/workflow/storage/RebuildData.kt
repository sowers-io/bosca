package io.bosca.workflow.storage

import com.meilisearch.sdk.Client
import io.bosca.graphql.GetStorageSystemsQuery
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.StorageSystemType
import io.bosca.util.decode
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import io.bosca.workflow.search.Configuration
import io.bosca.workflow.search.IndexConfiguration
import io.bosca.workflow.search.suspendWaitForTask

class RebuildData(client: io.bosca.api.Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Rebuild Storage Data",
            description = "Clear Storage Data",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    private suspend fun deleteAll(system: GetStorageSystemsQuery.All) {
        when (system.storageSystem.type) {
            StorageSystemType.SEARCH -> {
                val meilisearchConfig = client.configurations.get<Configuration>("meilisearch")
                    ?: error("meilisearch configuration missing")
                val client = Client(meilisearchConfig.toConfig())
                val cfg = system.storageSystem.configuration.decode<IndexConfiguration>()
                    ?: error("index configuration missing")
                val index = client.index(cfg.name)
                val taskId = index.deleteAllDocuments().taskUid
                index.suspendWaitForTask(taskId)
            }

            StorageSystemType.VECTOR -> {

            }

            else -> {
            }
        }
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val storageSystems = client.workflows.getStorageSystems()

        for (system in storageSystems) {
            deleteAll(system)
        }

        var offset = 0
        do {
            val metadatas = client.metadata.getAll(offset, 100)
            if (metadatas.isEmpty()) break
            for (metadata in metadatas) {
                client.workflows.enqueueMetadataWorkflow(
                    "metadata.update.storage",
                    metadata.id,
                    metadata.version
                )
            }
            offset += 100
        } while (true)

        offset = 0
        do {
            val collections = client.collections.getAll(offset, 100)
            if (collections.isEmpty()) break
            for (collection in collections) {
                client.workflows.enqueueCollectionWorkflow(
                    "collection.update.storage",
                    collection.id
                )
            }
            offset += 100
        } while (true)
    }

    companion object {
        const val ID = "storage.rebuild.data"
    }
}