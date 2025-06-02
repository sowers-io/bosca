package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.coroutines.delay
import kotlinx.serialization.Serializable

@Serializable
data class PublishRelationshipsConfiguration(
    val state: String? = null,
    val public: Boolean? = null,
    val publicContent: Boolean? = null,
    val publicSupplementary: Boolean? = null,
)

class PublishRelationships(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Publish Relationships",
            description = "Publish Relationships",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val configuration = getConfiguration<PublishRelationshipsConfiguration>(job)
        val state = configuration.state ?: "published"
        job.metadata?.metadata?.let {
            client.metadata.getRelationships(it.id).forEach { relationship ->
                val metadata = relationship.metadata.metadataRelationshipMetadata
                if (it.ready == null) {
                    client.metadata.setReady(it.id)
                    var tries = 10
                    while (tries-- > 0) {
                        val updated = client.metadata.get(it.id)
                        if (updated == null) throw Exception("metadata not found while trying to update workflow state")
                        if (updated.workflow.metadataWorkflow.state == state) break
                        delay(5000)
                    }
                }
                if (configuration.public == true && !it.public) {
                    client.metadata.setPublic(it.id, true)
                }
                if (configuration.publicContent == true && !metadata.publicContent) {
                    client.metadata.setPublicContent(it.id, true)
                }
                if (configuration.publicSupplementary == true && !metadata.publicSupplementary) {
                    client.metadata.setPublicSupplementary(it.id, true)
                }
                client.workflows.beginMetadataTransition(
                    metadata.id,
                    metadata.version,
                    state,
                    "Publishing Relationship from Workflow",
                    restart = true
                )
            }
        }
        job.collection?.collection?.let {
            client.collections.getRelationships(it.id).forEach { relationship ->
                val metadata = relationship.metadata.metadataRelationshipMetadata
                if (it.ready == null) {
                    client.metadata.setReady(it.id)
                    var tries = 10
                    while (tries-- > 0) {
                        val updated = client.metadata.get(it.id)
                        if (updated == null) throw Exception("metadata not found while trying to update workflow state")
                        if (updated.workflow.metadataWorkflow.state == state) break
                        delay(5000)
                    }
                }
                if (configuration.public == true && !it.public) {
                    client.metadata.setPublic(it.id, true)
                }
                if (configuration.publicContent == true && !metadata.publicContent) {
                    client.metadata.setPublicContent(it.id, true)
                }
                if (configuration.publicSupplementary == true && !metadata.publicSupplementary) {
                    client.metadata.setPublicSupplementary(it.id, true)
                }
                client.workflows.beginMetadataTransition(
                    metadata.id,
                    metadata.version,
                    state,
                    "Publishing Relationship from Workflow",
                    restart = true
                )
            }
        }
    }

    companion object {

        const val ID = "content.publish.relationships"
    }
}