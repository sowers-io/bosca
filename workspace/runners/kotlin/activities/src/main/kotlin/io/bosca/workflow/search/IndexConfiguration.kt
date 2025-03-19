package io.bosca.workflow.search

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class IndexConfiguration(
    @SerialName("indexName")
    val name: String,
    val primaryKey: String = "id",
    val filterable: List<String> = emptyList(),
    val sortable: List<String> = emptyList(),
    val searchable: List<String> = emptyList(),
)