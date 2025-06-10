package io.bosca.workflow.search

import com.meilisearch.sdk.Client
import com.meilisearch.sdk.Index
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.ActivityParameterInput
import io.bosca.graphql.type.ActivityParameterType
import io.bosca.graphql.type.StorageSystemType
import io.bosca.util.decode
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext

class AddToIndexes(client: io.bosca.api.Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Add Item to Search Indexes",
            description = "Add Item to Search Indexes",
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
        val meilisearchConfig = newMeilisearchConfig()
        val storageSystems = client.workflows.getStorageSystems()
        val client = Client(meilisearchConfig)
        val document = getInputSupplementaryText(context, job, INPUT_NAME)
        val taskIds = mutableListOf<Pair<Index, Int>>()
        for (system in storageSystems.filter { it.type == StorageSystemType.SEARCH }) {
            val cfg = system.configuration.decode<IndexConfiguration>() ?: error("index configuration missing")
            val index = client.index(cfg.name)
            val taskId = index.addDocuments(document).taskUid
            taskIds.add(index to taskId)
        }
        for ((index, taskId) in taskIds) {
            index.suspendWaitForTask(taskId)
        }
    }

    companion object {
        const val ID = "search.indexes.item"
        const val INPUT_NAME = "supplementary"
    }
}