package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.WorkflowStateType
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.coroutines.delay
import kotlinx.serialization.Serializable

@Serializable
data class PublishMetadataConfiguration(
    val public: Boolean = true,
    val publicContent: Boolean = true,
    val publicSupplementary: Boolean? = null,
)

class PublishMetadata(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Publish Metadata",
            description = "Publish Metadata",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val metadata = job.metadata?.metadata ?: return
        val configuration = getConfiguration<PublishMetadataConfiguration>(job)
        if (metadata.ready == null) {
            client.metadata.setReady(metadata.id)
            val draft = client.workflows.getStates().first { it.type == WorkflowStateType.DRAFT }
            var tries = 100
            while (tries-- > 0) {
                val updated = client.metadata.get(metadata.id)
                if (updated == null) throw Exception("metadata not found while trying to update workflow state")
                if (updated.workflow.metadataWorkflow.state == draft.id) break
                delay(1000)
            }
        }
        if (!metadata.public && configuration.public) {
            client.metadata.setPublic(metadata.id, true)
        }
        if (!metadata.publicContent && configuration.publicContent) {
            client.metadata.setPublicContent(metadata.id, true)
        }
        if (configuration.publicSupplementary != null) {
            if (!metadata.publicSupplementary && configuration.publicSupplementary) {
                client.metadata.setPublicSupplementary(metadata.id, true)
            }
        } else if (metadata.content.metadataContent.type.startsWith("image/")) {
            // TODO: enable permissions on an individual basis
            client.metadata.setPublicSupplementary(metadata.id, true)
        }
        if (!metadata.locked) {
            client.metadata.setLocked(metadata.id, metadata.version, true)
        }
    }

    companion object {
        const val ID = "metadata.publish"
    }
}