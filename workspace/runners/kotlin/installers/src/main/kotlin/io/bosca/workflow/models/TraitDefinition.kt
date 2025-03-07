package io.bosca.workflow.models

import kotlinx.serialization.Serializable

@Serializable
data class TraitDefinition(
    val id: String,
    val name: String,
    val description: String,
    val workflowIds: List<String>,
    val contentTypes: List<String>
)