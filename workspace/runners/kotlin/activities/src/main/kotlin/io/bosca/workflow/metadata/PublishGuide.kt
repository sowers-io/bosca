package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
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
        val guide = client.metadata.getGuide(metadata.id, metadata.version) ?: return
        for (step in guide.steps) {
            step.guideStep.metadata?.metadata?.let {
                if (it.workflow.metadataWorkflow.state == "published") return@let
                client.workflows.beginMetadataTransition(
                    it.id,
                    it.version,
                    "published",
                    "Publishing Guide from Workflow",
                    true
                )
            }
            for (module in step.guideStep.modules) {
                module.guideStepModule.metadata?.metadata?.let {
                    if (it.workflow.metadataWorkflow.state == "published") return@let
                    client.workflows.beginMetadataTransition(
                        it.id,
                        it.version,
                        "published",
                        "Publishing Guide Module from Workflow",
                        true
                    )
                }
            }
        }
    }

    companion object {
        const val ID = "metadata.publish.guide"
    }
}