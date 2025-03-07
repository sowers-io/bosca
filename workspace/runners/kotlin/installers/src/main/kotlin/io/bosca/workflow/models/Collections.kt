package io.bosca.workflow.models

import kotlinx.serialization.Serializable

@Serializable
data class Collections(
    val collections: List<CollectionDefinition>,
)

