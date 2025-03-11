package io.bosca.workflow.models

import io.bosca.workflow.yaml.YamlAnyDeserializer
import kotlinx.serialization.Serializable

@Serializable
data class StateDefinition(
    val id: String,
    val name: String,
    val description: String,
    val type: String,
    val configuration: @Serializable(YamlAnyDeserializer::class) Any? = null,
    val workflowId: String? = null,
    val exitWorkflowId: String? = null,
    val entryWorkflowId: String? = null
)