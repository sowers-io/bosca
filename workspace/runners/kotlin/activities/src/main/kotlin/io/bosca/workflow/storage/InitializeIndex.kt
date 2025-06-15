package io.bosca.workflow.storage

import com.meilisearch.sdk.exceptions.MeilisearchApiException
import com.meilisearch.sdk.model.Embedder
import com.meilisearch.sdk.model.EmbedderSource
import io.bosca.api.Client
import io.bosca.api.executeAsync
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.StorageSystemType
import io.bosca.models.OpenAIConfiguration
import io.bosca.util.decode
import io.bosca.util.json
import io.bosca.util.toAny
import io.bosca.util.toJsonElement
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import io.bosca.workflow.search.ExperimentalConfiguration
import io.bosca.workflow.search.IndexConfiguration
import io.bosca.workflow.search.MeilisearchConfiguration
import io.bosca.workflow.search.newMeilisearchConfig
import io.bosca.workflow.search.suspendWaitForTask
import kotlinx.serialization.Serializable
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.Request
import okhttp3.RequestBody.Companion.toRequestBody

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
        val meilisearchConfiguration = this.client.configurations.get<MeilisearchConfiguration>("meilisearch")
        val openAIConfiguration = this.client.configurations.get<OpenAIConfiguration>("openai")
        val openAIKey = openAIConfiguration?.key
        val configuration = getConfiguration<InitializeIndexConfiguration>(job)
        val systems = if (configuration.id != null) {
            val system = client.workflows.getStorageSystem(configuration.id)
            system?.let { listOf(it) } ?: emptyList()
        } else {
            job.storageSystems.filter { it.system.storageSystem.type == StorageSystemType.SEARCH }
                .map { it.system.storageSystem }
        }
        val meilisearchConfig = newMeilisearchConfig(meilisearchConfiguration)
        val client = com.meilisearch.sdk.Client(meilisearchConfig)
        meilisearchConfiguration?.experimental?.let {
            @Suppress("UNCHECKED_CAST")
            client.experimentalFeatures(json.encodeToJsonElement(ExperimentalConfiguration.serializer(), it).toAny() as Map<String, Boolean>)
        }
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
            settings.embedders = (settings.embedders ?: HashMap())
            for (embedder in cfg.embedders) {
                val source = EmbedderSource.entries.first { it.source == embedder.source }
                var key = embedder.apiKey
                if (key == null && openAIKey != null && embedder.source == EmbedderSource.OPEN_AI.source) {
                    key = openAIKey
                }
                settings.embedders.put(embedder.name, Embedder.builder().apply {
                    this.source(source)
                    this.apiKey(key)
                    this.model(embedder.model)
                    this.dimensions(embedder.dimensions)
                    this.documentTemplate(embedder.documentTemplate)
                    this.documentTemplateMaxBytes(embedder.documentTemplateMaxBytes)
                }.build())
            }
            val task = index.updateSettings(settings)
            index.suspendWaitForTask(task.taskUid)
            for (chat in cfg.chatSettings) {
                val prompts = mutableMapOf<String, String>()
                var key = chat.apiKey
                if (key == null && openAIKey != null && chat.source == "openAi") {
                    key = openAIKey
                }
                val settings = mapOf(
                    "source" to chat.source,
                    "apiKey" to key,
                    "prompts" to prompts
                )
                chat.prompts?.let {
                    prompts["system"] = it.system
                }
                this.client.network.http.newCall(
                    Request.Builder()
                        .url("${meilisearchConfig.hostUrl}/chats/${chat.name}/settings")
                        .header("Authorization", "Bearer ${meilisearchConfig.apiKey}")
                        .patch(settings.toJsonElement().toString().toRequestBody("application/json".toMediaType()))
                        .build()
                ).executeAsync()
            }
        }
    }

    companion object {

        const val ID = "storage.index.initialize"
    }
}