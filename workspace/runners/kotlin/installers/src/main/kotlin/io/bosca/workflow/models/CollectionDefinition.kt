package io.bosca.workflow.models

import io.bosca.graphql.type.AttributeType
import io.bosca.graphql.type.CollectionType
import io.bosca.graphql.type.Order
import kotlinx.serialization.Serializable

@Serializable
data class Ordering(
    val order: Order = Order.ASCENDING,
    val type: AttributeType? = null,
    val field: String? = null,
    val path: List<String>? = null
)

@Serializable
data class CollectionPermissionDefinition(
    val group: String,
    val action: String
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
    val permissions: List<CollectionPermissionDefinition>? = null,
    val ordering: List<Ordering>? = null
)

@Serializable
data class CollectionTemplates(
    val collection: CollectionTemplateDefinition? = null,
    val document: DocumentTemplateDefinition? = null,
    val guide: GuideTemplateDefinition? = null
)
