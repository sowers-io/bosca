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

@Serializable
data class AddToIndexContext(val taskId: Int? = null)

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
        val jobContext = getContext<AddToIndexContext>(job)
        val meilisearchConfig = newMeilisearchConfig()
        val configuration = getConfiguration<AddToIndexConfiguration>(job)
        val storageSystem = this.client.workflows.getStorageSystems().firstOrNull { it.name == configuration.storageSystem } ?: error("storage system missing")
        val client = Client(meilisearchConfig)
        val document = getInputSupplementaryText(context, job, INPUT_NAME)
        val cfg = storageSystem.configuration.decode<IndexConfiguration>() ?: error("index configuration missing")
        val index = client.index(cfg.name)
        if (jobContext.taskId != null) {
            try {
                index.suspendWaitForTask(jobContext.taskId)
                return
            } catch (_: TaskFailedException) {
                println("task failed: $jobContext, retrying")
            }
        }
        val taskId = index.addDocuments(document).taskUid
        setContext(job, AddToIndexContext(taskId))
        index.suspendWaitForTask(taskId)
    }

    companion object {
        const val ID = "search.index.item"
        const val INPUT_NAME = "supplementary"
    }
}