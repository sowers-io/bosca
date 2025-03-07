package io.bosca.workflow.models

import kotlinx.serialization.Serializable

@Serializable
data class AttributeDefinition(
    val key: String = "",
    val name: String = "",
    val description: String = "",
    val workflows: List<WorkflowReferenceDefinition> = emptyList(),
    val type: String = "",
    val supplementaryKey: String? = null,
    val ui: String = "",
    val list: Boolean = false,
    val configuration: Map<String, String>? = null
)
