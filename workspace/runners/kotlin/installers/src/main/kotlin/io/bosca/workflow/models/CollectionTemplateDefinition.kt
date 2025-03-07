package io.bosca.workflow.models

import kotlinx.serialization.Serializable

@Serializable
data class CollectionTemplateDefinition(
    val name: String = "",
    val description: String = "",
    val collection: CollectionTemplateCollection = CollectionTemplateCollection()
)

@Serializable
data class CollectionTemplateCollection(
    val defaultAttributes: Map<String, String> = emptyMap(),
    val attributes: List<AttributeDefinition> = emptyList(),
    val configuration: Map<String, String> = emptyMap(),
    val collectionsEnabled: Boolean = true,
    val collectionsFilter: FindQueriesDefinition? = null,
    val metadataEnabled: Boolean = true,
    val metadataFilter: FindQueriesDefinition? = null
)