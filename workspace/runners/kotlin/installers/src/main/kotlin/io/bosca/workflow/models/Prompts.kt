package io.bosca.workflow.models

import kotlinx.serialization.Serializable

@Serializable
data class Prompts(
    val prompts: List<PromptDefinition>
)