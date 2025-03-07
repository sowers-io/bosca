package io.bosca.workflow.models

import kotlinx.serialization.Serializable

@Serializable
data class WorkflowReferenceDefinition(
    val autoRun: Boolean = false,
    val workflow: String = ""
)