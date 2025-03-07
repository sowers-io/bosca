package io.bosca.workflow.models

import kotlinx.serialization.Serializable

@Serializable
data class DocumentTemplateDefinition(
    val name: String = "",
    val description: String = "",
    val document: DocumentTemplateDocument = DocumentTemplateDocument()
)

@Serializable
data class DocumentTemplateDocument(
    val configuration: Map<String, String> = emptyMap(),
    val content: DocumentTemplateContent = DocumentTemplateContent(),
    val defaultAttributes: Map<String, String> = emptyMap(),
    val attributes: List<AttributeDefinition> = emptyList(),
    val containers: List<DocumentTemplateContainer> = emptyList(),
)

@Serializable
data class DocumentTemplateContainer(
    val id: String = "",
    val name: String = "",
    val description: String = "",
    val supplementary: String? = null,
    val workflows: List<WorkflowReferenceDefinition> = emptyList(),
)

@Serializable
data class DocumentTemplateContent(
    val document: DocumentTemplateContentDocument = DocumentTemplateContentDocument()
)

@Serializable
data class DocumentTemplateContentDocument(
    val content: List<ContentNode> = emptyList()
)

@Serializable
data class ContentNode(
    val type: String = "",
    val attributes: Map<String, String>? = null,
    val content: List<ContentNode>? = null,
    val text: String? = null
)