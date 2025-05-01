package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext

data class PublishGuideConfiguration(
    val state: String? = null,
    val public: Boolean? = null,
    val publicContent: Boolean? = null,
    val publicSupplementary: Boolean? = null,
)

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
        val configuration = getConfiguration<PublishGuideConfiguration>(job)
        val state = configuration.state ?: "published"
        val guide = client.metadata.getGuide(metadata.id, metadata.version) ?: return
        for (step in guide.steps) {
            step.guideStep.metadata?.metadata?.let {
                if (it.workflow.metadataWorkflow.state == state) return@let
                client.workflows.beginMetadataTransition(
                    it.id,
                    it.version,
                    state,
                    "Publishing Guide from Workflow",
                    true
                )
                if (configuration.public == true && !it.public) {
                    client.metadata.setPublic(it.id, true)
                }
                if (configuration.publicContent == true && !it.publicContent) {
                    client.metadata.setPublicContent(it.id, true)
                }
                if (configuration.publicSupplementary == true && !it.publicSupplementary) {
                    client.metadata.setPublicSupplementary(it.id, true)
                }
            }
            for (module in step.guideStep.modules) {
                module.guideStepModule.metadata?.metadata?.let {
                    if (it.workflow.metadataWorkflow.state == state) return@let
                    client.workflows.beginMetadataTransition(
                        it.id,
                        it.version,
                        state,
                        "Publishing Guide Module from Workflow",
                        true
                    )
                    if (configuration.public == true && !it.public) {
                        client.metadata.setPublic(it.id, true)
                    }
                    if (configuration.publicContent == true && !it.publicContent) {
                        client.metadata.setPublicContent(it.id, true)
                    }
                    if (configuration.publicSupplementary == true && !it.publicSupplementary) {
                        client.metadata.setPublicSupplementary(it.id, true)
                    }
                }
            }
        }
    }

    companion object {
        const val ID = "metadata.publish.guide"
    }
}