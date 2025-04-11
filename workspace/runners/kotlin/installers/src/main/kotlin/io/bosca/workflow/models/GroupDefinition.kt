package io.bosca.workflow.models

import kotlinx.serialization.Serializable

@Serializable
data class GroupDefinition(
    val name: String,
    val description: String,
    val permissions: List<GroupPermissionDefinition>? = null
)

@Serializable
data class GroupPermissionDefinition(
    val slug: String,
    val action: String
)