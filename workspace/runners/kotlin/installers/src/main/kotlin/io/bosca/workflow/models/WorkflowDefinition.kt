package io.bosca.workflow.models

import io.bosca.yaml.YamlAnyDeserializer
import kotlinx.serialization.Serializable

@Serializable
data class WorkflowDefinition(
    val id: String,
    val name: String,
    val description: String,
    val queue: String,
    val configuration: @Serializable(YamlAnyDeserializer::class) Any? = null,
    val activities: List<ActivityDefinition>
)
