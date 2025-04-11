package io.bosca.workflow.models

import kotlinx.serialization.Serializable

@Serializable
data class Groups(
    val groups: List<GroupDefinition>
)