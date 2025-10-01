package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.WorkflowStateType
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.coroutines.delay

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
        val states = client.workflows.getStates()
        val state = states.first { it.type == WorkflowStateType.PUBLISHED }
        val draft = states.first { it.type == WorkflowStateType.DRAFT }
        job.metadata?.metadata?.let {
            for (relationship in client.metadata.getRelationships(it.id)) {
                val m = relationship.metadata.metadataRelationshipMetadata
                if (m.ready == null) {
                    client.metadata.setReady(m.id)
                    var tries = 100
                    while (tries-- > 0) {
                        val updated = client.metadata.get(m.id)
                        if (updated == null) throw Exception("metadata not found while trying to update workflow state")
                        if (updated.metadataWorkflow?.metadataWorkflow?.state == draft.id) break
                        delay(1000)
                    }
                }
                if (!m.public) {
                    client.metadata.setPublic(m.id, true)
                }
                if (!m.publicContent) {
                    client.metadata.setPublicContent(m.id, true)
                }
                if (!m.publicSupplementary && m.content.type.startsWith("image/")) {
                    client.metadata.setPublicSupplementary(m.id, true)
                }
                if (m.workflow?.state != state.id) {
                    if (m.workflow?.pending != null) {
                        client.workflows.cancelMetadataTransition(m.id, m.version)
                    }
                    client.workflows.beginMetadataTransition(
                        m.id,
                        m.version,
                        state.id,
                        "Published with Metadata"
                    )
                }
            }
        }
        job.collection?.collection?.let {
            for (relationship in client.collections.getRelationships(it.id)) {
                val m = relationship.metadata.metadataRelationshipMetadata
                if (m.ready == null) {
                    client.metadata.setReady(m.id)
                    var tries = 100
                    while (tries-- > 0) {
                        val updated = client.metadata.get(m.id)
                        if (updated == null) throw Exception("metadata not found while trying to update workflow state")
                        if (updated.metadataWorkflow?.metadataWorkflow?.state == draft.id) break
                        delay(1000)
                    }
                }
                if (!m.public) {
                    client.metadata.setPublic(m.id, true)
                }
                if (!m.publicContent) {
                    client.metadata.setPublicContent(m.id, true)
                }
                if (!m.publicSupplementary && m.content.type.startsWith("image/")) {
                    client.metadata.setPublicSupplementary(m.id, true)
                }
                if (m.workflow?.state != state.id) {
                    if (m.workflow?.pending != null) {
                        client.workflows.cancelMetadataTransition(m.id, m.version)
                    }
                    client.workflows.beginMetadataTransition(
                        m.id,
                        m.version,
                        state.id,
                        "Published with Collection"
                    )
                }
            }
        }
    }

    companion object {
        const val ID = "content.publish.relationships"
    }
}