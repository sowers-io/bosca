package io.bosca.workflow.models

import io.bosca.yaml.YamlAnyDeserializer
import kotlinx.serialization.Serializable

@Serializable
data class ActivityDefinition(
    val queue: String,
    val activity: String,
    val description: String? = null,
    val executionGroup: Int? = null,
    val configuration: @Serializable(YamlAnyDeserializer::class) Any? = null,
    val inputs: List<ParameterDefinition> = emptyList(),
    val outputs: List<ParameterDefinition> = emptyList(),
    val models: List<ModelReferenceDefinition> = emptyList(),
    val prompts: List<PromptReferenceDefinition> = emptyList(),
    val storageSystems: List<StorageSystemReferenceDefinition> = emptyList()
)