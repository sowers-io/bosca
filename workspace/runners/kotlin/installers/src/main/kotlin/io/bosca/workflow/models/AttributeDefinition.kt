package io.bosca.workflow.models

import io.bosca.workflow.yaml.YamlAnyDeserializer
import kotlinx.serialization.Serializable

enum class AttributeLocation {
    ITEM,
    RELATIONSHIP
}

@Serializable
data class AttributeDefinition(
    val key: String = "",
    val name: String = "",
    val description: String = "",
    val workflows: List<WorkflowReferenceDefinition> = emptyList(),
    val type: String = "",
    val supplementaryKey: String? = null,
    val ui: String = "",
    val list: Boolean = false,
    val configuration: @Serializable(YamlAnyDeserializer::class) Any? = null,
    val location: AttributeLocation? = null
)
