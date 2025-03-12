package io.bosca.workflow.models

import io.bosca.workflow.yaml.YamlAnyDeserializer
import kotlinx.serialization.Serializable

@Serializable
data class ConfigurationDefinition(
    val key: String,
    val description: String,
    val public: Boolean,
    val value: @Serializable(YamlAnyDeserializer::class) Any
)