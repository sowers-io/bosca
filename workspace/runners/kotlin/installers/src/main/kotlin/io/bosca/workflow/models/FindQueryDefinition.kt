package io.bosca.workflow.models

import io.bosca.graphql.type.CollectionType
import io.bosca.graphql.type.ExtensionFilterType
import kotlinx.serialization.Serializable

@Serializable
data class FindAttributes(val attributes: Map<String, String>)

@Serializable
data class FindQueryDefinition(
    val attributes: List<FindAttributes> = emptyList(),
    val contentTypes: List<String> = emptyList(),
    val categoryIds: List<String> = emptyList(),
    val extensionFilter: ExtensionFilterType? = null,
    val collectionType: CollectionType? = null
)

@Serializable
data class FindQueryOption(
    val name: String,
    val query: FindQueryDefinition
)

@Serializable
data class FindQueriesDefinition(
    val options: List<FindQueryOption>
)