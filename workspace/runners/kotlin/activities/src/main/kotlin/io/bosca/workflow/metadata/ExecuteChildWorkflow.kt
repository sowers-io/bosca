package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.util.encodeToOptional
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.serialization.Serializable

@Serializable
data class ExecuteChildWorkflowConfig(val workflows: List<String>)

class ExecuteChildWorkflow(client: Client) : Activity(client) {
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
            config.workflows,
            job.id,
        )
    }

    companion object {
        const val ID = "workflow.execute.child"
    }
}