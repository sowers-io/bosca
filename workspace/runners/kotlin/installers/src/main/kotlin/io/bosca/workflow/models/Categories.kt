package io.bosca.workflow.models

import kotlinx.serialization.Serializable

@Serializable
data class Categories(
    val categories: List<String>,
)

