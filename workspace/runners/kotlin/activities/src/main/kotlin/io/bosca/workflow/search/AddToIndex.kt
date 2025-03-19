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
data class AddToIndexConfiguration(val storageSystem: String)

class AddToIndex(client: io.bosca.api.Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Add Item to Search Index",
            description = "Add Item to Search Index",
            inputs = listOf(
                ActivityParameterInput(
                    INPUT_NAME,
                    ActivityParameterType.SUPPLEMENTARY,
                )
            ),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val meilisearchConfig = client.configurations.get<Configuration>("meilisearch") ?: error("meilisearch configuration missing")
        val configuration = getConfiguration<AddToIndexConfiguration>(job)
        val storageSystem = client.workflows.getStorageSystems().firstOrNull { it.storageSystem.name == configuration.storageSystem } ?: error("storage system missing")
        val client = Client(meilisearchConfig.toConfig())
        val document = getInputSupplementaryText(context, job, INPUT_NAME)
        val cfg = storageSystem.storageSystem.configuration.decode<IndexConfiguration>() ?: error("index configuration missing")
        val index = client.index(cfg.name)
        val taskId = index.addDocuments(document).taskUid
        index.suspendWaitForTask(taskId)
    }

    companion object {
        const val ID = "search.index.item"
        const val INPUT_NAME = "supplementary"
    }
}