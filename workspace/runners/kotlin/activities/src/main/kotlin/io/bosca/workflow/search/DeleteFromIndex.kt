package io.bosca.workflow.search

import com.meilisearch.sdk.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.ActivityParameterInput
import io.bosca.graphql.type.ActivityParameterType
import io.bosca.util.decode
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.serialization.Serializable

@Serializable
data class DeleteFromConfiguration(val storageSystem: String)

class DeleteFromIndex(client: io.bosca.api.Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Delete Item from Search Index",
            description = "Delete Item from Search Index",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val meilisearchConfig = client.configurations.get<Configuration>("meilisearch") ?: error("meilisearch configuration missing")
        val configuration = getConfiguration<AddToIndexConfiguration>(job)
        val storageSystem = client.workflows.getStorageSystems().firstOrNull { it.storageSystem.name == configuration.storageSystem }
                ?: error("storage system missing")
        val client = Client(meilisearchConfig.toConfig())
        val cfg = storageSystem.storageSystem.configuration.decode<IndexConfiguration>()
            ?: error("index configuration missing")
        val index = client.index(cfg.name)
        val taskId = index.deleteDocument(
            job.metadata?.metadata?.id ?: job.collection?.collection?.id ?: job.profile?.profile?.id
            ?: error("missing id")
        ).taskUid
        index.suspendWaitForTask(taskId)
    }

    companion object {
        const val ID = "search.index.item.delete"
    }
}