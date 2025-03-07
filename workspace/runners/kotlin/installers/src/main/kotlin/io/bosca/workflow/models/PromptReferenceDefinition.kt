package io.bosca.workflow.models

import kotlinx.serialization.Serializable

@Serializable
data class PromptReferenceDefinition(
    val name: String
)