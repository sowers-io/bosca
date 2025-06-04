package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.WorkflowStateType
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext

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
        val published = client.workflows.getStates().first { it.type == WorkflowStateType.PUBLISHED }
        job.metadata?.metadata?.let { metadata ->
            val relationships = client.metadata.getRelationships(metadata.id)
            for (relationship in relationships) {
                if (relationship.metadata.metadataRelationshipMetadata.workflow.pending == null &&
                    relationship.metadata.metadataRelationshipMetadata.workflow.state != published.id
                ) {
                    client.workflows.beginMetadataTransition(
                        relationship.metadata.metadataRelationshipMetadata.id,
                        relationship.metadata.metadataRelationshipMetadata.version,
                        published.id,
                        "Publishing Relationship for ${metadata.id}",
                    )
                }
            }
        }
        job.collection?.collection?.let { collection ->
            val relationships = client.collections.getRelationships(collection.id)
            for (relationship in relationships) {
                if (relationship.metadata.metadataRelationshipMetadata.workflow.pending == null &&
                    relationship.metadata.metadataRelationshipMetadata.workflow.state != published.id
                ) {
                    client.workflows.beginMetadataTransition(
                        relationship.metadata.metadataRelationshipMetadata.id,
                        relationship.metadata.metadataRelationshipMetadata.version,
                        published.id,
                        "Publishing Relationship for ${collection.id}",
                    )
                }
            }
        }
    }

    companion object {

        const val ID = "content.publish.relationships"
    }
}