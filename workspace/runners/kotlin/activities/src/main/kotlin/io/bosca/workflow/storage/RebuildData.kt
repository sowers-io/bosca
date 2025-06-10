package io.bosca.workflow.storage

import com.meilisearch.sdk.Client
import io.bosca.graphql.GetStorageSystemsQuery
import io.bosca.graphql.fragment.StorageSystem
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.StorageSystemType
import io.bosca.util.decode
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import io.bosca.workflow.search.IndexConfiguration
import io.bosca.workflow.search.newMeilisearchConfig
import io.bosca.workflow.search.suspendWaitForTask
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.serialization.Serializable

@Serializable
class RebuildDataConfiguration(
    val id: String? = null
)

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

    private suspend fun deleteAll(system: StorageSystem) {
        when (system.type) {
            StorageSystemType.SEARCH -> {
                val meilisearchConfig = newMeilisearchConfig()
                val client = Client(meilisearchConfig)
                val cfg = system.configuration.decode<IndexConfiguration>()
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

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) = coroutineScope {
        val configuration = getConfiguration<RebuildDataConfiguration>(job)
        val storageSystems = if (configuration.id != null) {
            client.workflows.getStorageSystem(configuration.id)?.let { listOf(it) } ?: emptyList()
        } else {
            client.workflows.getStorageSystems()
        }

        for (system in storageSystems) {
            deleteAll(system)
        }

        val metadatas = launch {
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
        }

        val collections = launch {
            var offset = 0
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

        val profiles = launch {
            var offset = 0
            do {
                val profiles = client.profiles.getAll(offset, 100)
                if (profiles.isEmpty()) break
                for ((profile, _) in profiles) {
                    client.workflows.enqueueProfileWorkflow(
                        "profile.update.storage",
                        profile.id
                    )
                }
                offset += 100
            } while (true)
        }

        arrayOf(metadatas, collections, profiles).forEach { it.join() }
    }

    companion object {
        const val ID = "storage.rebuild.data"
    }
}