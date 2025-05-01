package io.bosca.workflow.models

import io.bosca.yaml.YamlAnyDeserializer
import kotlinx.serialization.Serializable

@Serializable
data class AssetFolder(
    val slug: String,
    val name: String,
    val description: String? = null,
    val folders: List<AssetFolder> = emptyList(),
    val assets: List<AssetDefinition> = emptyList(),
    val public: Boolean = true,
    val ready: Boolean = true
)

@Serializable
data class AssetDefinition(
    val slug: String,
    val name: String,
    val file: String? = null,
    val description: String? = null,
    val mimeType: String? = null,
    val public: Boolean = true,
    val publicContent: Boolean = true,
    val publish: Boolean = true,
    val attributes: @Serializable(YamlAnyDeserializer::class) Any? = null,
    val ready: Boolean = true
)

@Serializable
data class AssetsDefinition(
    val folders: List<AssetFolder>,
    val assets: List<AssetDefinition>
)