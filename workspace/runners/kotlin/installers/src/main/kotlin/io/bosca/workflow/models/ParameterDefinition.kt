package io.bosca.workflow.models

import kotlinx.serialization.Serializable

@Serializable
data class ParameterDefinition(
    val name: String,
    val supplementary: String
)