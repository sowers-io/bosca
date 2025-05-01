package io.bosca.workflow.models

import io.bosca.graphql.type.StorageSystemType
import io.bosca.yaml.YamlAnyDeserializer
import kotlinx.serialization.Serializable

@Serializable
data class StorageSystemDefinition(
    val name: String,
    val description: String,
    val type: StorageSystemType,
    val configuration: @Serializable(YamlAnyDeserializer::class) Any? = null,
    val models: List<StorageSystemModelDefinition>
)

@Serializable
data class StorageSystemModelDefinition(
    val model: String,
    val configuration: @Serializable(YamlAnyDeserializer::class) Any = emptyMap<String, Any>()
)
