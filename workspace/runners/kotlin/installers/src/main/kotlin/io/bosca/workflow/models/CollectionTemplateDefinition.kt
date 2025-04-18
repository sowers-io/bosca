package io.bosca.workflow.models

import kotlinx.serialization.Serializable

@Serializable
data class CollectionTemplateDefinition(
    val name: String = "",
    val description: String = "",
    val permissions: List<CollectionPermissionDefinition>? = null,
    val collection: CollectionTemplateCollection = CollectionTemplateCollection(),
    val ordering: List<Ordering>? = null
)

@Serializable
data class CollectionTemplateCollection(
    val defaultAttributes: Map<String, String> = emptyMap(),
    val attributes: List<AttributeDefinition> = emptyList(),
    val configuration: Map<String, String> = emptyMap(),
    val filters: CollectionTemplateFilters? = null,
    val ordering: List<Ordering>? = null
)

@Serializable
data class CollectionTemplateFilters(
    val filters: List<CollectionTemplateFilter> = emptyList()
)

@Serializable
data class CollectionTemplateFilter(
    val name: String = "",
    val filter: String = ""
)