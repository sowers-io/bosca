package io.bosca.workflow.models

import kotlinx.serialization.Serializable

@Serializable
data class Configurations(
    val configurations: List<ConfigurationDefinition>
)

