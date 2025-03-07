package io.bosca.workflow.models

import io.bosca.graphql.type.AttributeType
import io.bosca.graphql.type.CollectionType
import io.bosca.graphql.type.Order
import kotlinx.serialization.Serializable

@Serializable
data class Ordering(
    val order: Order,
    val path: List<String>,
    val type: AttributeType
)

@Serializable
data class CollectionDefinition(
    val slug: String,
    val name: String,
    val type: CollectionType,
    val attributes: Map<String, String> = emptyMap(),
    val editorType: String,
    val categories: List<String>? = null,
    val collections: List<CollectionDefinition>? = null,
    val templates: CollectionTemplates? = null,
    val ordering: List<Ordering>? = null
)

@Serializable
data class CollectionTemplates(
    val collection: CollectionTemplateDefinition? = null,
    val document: DocumentTemplateDefinition? = null,
    val guide: GuideTemplateDefinition? = null
)
