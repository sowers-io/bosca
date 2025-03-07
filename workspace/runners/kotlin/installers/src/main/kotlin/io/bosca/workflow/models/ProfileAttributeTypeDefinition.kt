package io.bosca.workflow.models

import io.bosca.graphql.type.ProfileVisibility
import kotlinx.serialization.Serializable

@Serializable
data class ProfileAttributeTypeDefinition(
    val id: String,
    val name: String,
    val description: String,
    val visibility: ProfileVisibility
)