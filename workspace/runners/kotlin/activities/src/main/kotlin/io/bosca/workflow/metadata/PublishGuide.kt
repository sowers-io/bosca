package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.WorkflowStateType
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext

class PublishGuide(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Publish Guide",
            description = "Publish Guide",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val metadata = job.metadata?.metadata ?: return
        if (metadata.content.metadataContent.type != "bosca/v-guide") return
        val published = client.workflows.getStates().first { it.type == WorkflowStateType.PUBLISHED }
        val guide = client.metadata.getGuide(metadata.id, metadata.version) ?: return
        for (step in guide.steps) {
            step.guideStep.metadata?.metadata?.let {
                if (it.workflow.metadataWorkflow.pending == null &&
                    it.workflow.metadataWorkflow.state != published.id
                ) {
                    client.workflows.beginMetadataTransition(
                        it.id,
                        it.version,
                        published.id,
                        "Publishing Guide Step for ${metadata.id}",
                    )
                }
            }
            for (module in step.guideStep.modules) {
                module.guideStepModule.metadata?.metadata?.let {
                    if (it.workflow.metadataWorkflow.pending == null &&
                        it.workflow.metadataWorkflow.state != published.id
                    ) {
                        client.workflows.beginMetadataTransition(
                            it.id,
                            it.version,
                            published.id,
                            "Publishing Guide Module for ${metadata.id}",
                        )
                    }
                }
            }
        }
    }

    companion object {
        const val ID = "metadata.publish.guide"
    }
}