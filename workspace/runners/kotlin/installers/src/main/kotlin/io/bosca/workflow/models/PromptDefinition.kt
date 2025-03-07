package io.bosca.workflow.models

import io.bosca.workflow.ai.schema.JsonSchema
import kotlinx.serialization.Serializable

@Serializable
data class PromptDefinition(
    val name: String,
    val description: String,
    val inputType: String,
    val outputType: String,
    val systemPrompt: String,
    val userPrompt: String,
    val schema: JsonSchema? = null
)
