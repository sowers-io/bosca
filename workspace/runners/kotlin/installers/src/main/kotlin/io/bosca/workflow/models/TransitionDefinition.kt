package io.bosca.workflow.models

import kotlinx.serialization.Serializable

@Serializable
data class TransitionDefinition(
    val fromState: String,
    val toState: String,
    val description: String
)