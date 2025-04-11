package io.bosca.workflow.models

import kotlinx.serialization.Serializable

@Serializable
data class GroupDefinition(
    val name: String,
    val description: String
)