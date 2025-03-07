package io.bosca.workflow.models

import kotlinx.serialization.Serializable

@Serializable
data class Traits(
    val traits: List<TraitDefinition>
)