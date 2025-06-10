package io.bosca.workflow.storage

import com.meilisearch.sdk.exceptions.MeilisearchApiException
import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.StorageSystemType
import io.bosca.util.decode
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import io.bosca.workflow.search.IndexConfiguration
import io.bosca.workflow.search.newMeilisearchConfig
import io.bosca.workflow.search.suspendWaitForTask
import kotlinx.serialization.Serializable

@Serializable
class InitializeIndexConfiguration(
    val id: String? = null
)

class InitializeIndex(client: Client) : Activity(client) {
    override val id = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Initialize an Index",
            description = "Initialize an Index",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val configuration = getConfiguration<InitializeIndexConfiguration>(job)
        val systems = if (configuration.id != null) {
            val system = client.workflows.getStorageSystem(configuration.id)
            system?.let { listOf(it) } ?: emptyList()
        } else {
            job.storageSystems.filter { it.system.storageSystem.type == StorageSystemType.SEARCH }
                .map { it.system.storageSystem }
        }
        val meilisearchConfig = newMeilisearchConfig()
        val client = com.meilisearch.sdk.Client(meilisearchConfig)
        for (system in systems) {
            val cfg = system.configuration.decode<IndexConfiguration>() ?: error("index configuration missing")
            val index = try {
                client.getIndex(cfg.name)
            } catch (_: MeilisearchApiException) {
                val task = client.createIndex(cfg.name, cfg.primaryKey)
                client.suspendWaitForTask(task.taskUid)
                client.getIndex(cfg.name)
            }
            val settings = index.settings
            settings.filterableAttributes = cfg.filterable.toTypedArray()
            settings.sortableAttributes = cfg.sortable.toTypedArray()
            settings.searchableAttributes = cfg.searchable.toTypedArray()
            val task = index.updateSettings(settings)
            index.suspendWaitForTask(task.taskUid)
        }
    }

    companion object {

        const val ID = "storage.index.initialize"
    }
}