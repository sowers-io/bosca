package io.bosca.workflow.models

import io.bosca.workflow.yaml.YamlAnyDeserializer
import kotlinx.serialization.Serializable

@Serializable
data class ModelDefinition(
    val name: String,
    val description: String,
    val type: String,
    val configuration: @Serializable(YamlAnyDeserializer::class) Any? = null
)
