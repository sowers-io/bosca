package io.bosca.workflow.metadata

import com.apollographql.apollo.api.Optional
import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.WorkflowActivityInput
import io.bosca.util.encodeToOptional
import io.bosca.util.toAny
import io.bosca.util.toOptional
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.encodeToJsonElement

@Serializable
data class ExecuteChildWorkflowConfig(val workflowIds: List<String>)

class ExecuteChildWorkflows(client: Client) : Activity(client) {
    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Execute Child Workflow",
            description = "",
            inputs = emptyList(),
            outputs = emptyList(),
            configuration = ExecuteChildWorkflowConfig(emptyList()).encodeToOptional()
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val config = getConfiguration<ExecuteChildWorkflowConfig>(job)
        client.workflows.enqueueChildWorkflows(
            config.workflowIds,
            job.id,
        )
    }

    companion object {
        const val ID = "workflow.execute.child"

        inline fun <reified T> newActivity(
            queue: String,
            description: String,
            executionGroup: Int,
            configuration: T
        ) =
            WorkflowActivityInput(
                activityId = ID,
                queue = queue,
                description = description,
                executionGroup = executionGroup,
                inputs = emptyList(),
                models = emptyList(),
                outputs = emptyList(),
                prompts = emptyList(),
                storageSystems = emptyList(),
                configuration = Optional.presentIfNotNull(
                    Json.encodeToJsonElement(configuration).toAny()
                )
            )
    }
}