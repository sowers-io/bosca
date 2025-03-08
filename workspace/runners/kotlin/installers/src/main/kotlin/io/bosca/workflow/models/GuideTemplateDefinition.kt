package io.bosca.workflow.models

import kotlinx.serialization.Serializable

@Serializable
data class GuideTemplateDefinition(
    val name: String = "",
    val description: String = "",
    val guide: GuideTemplateGuide = GuideTemplateGuide()
)

@Serializable
data class GuideTemplateGuide(
    val configuration: Map<String, String> = emptyMap(),
    val type: String = "",
    val rrule: String = "",
    val template: DocumentTemplateDefinition? = null,
    val steps: List<GuideTemplateStep> = emptyList(),
)

@Serializable
data class GuideTemplateStep(
    val template: DocumentTemplateDefinition? = null,
    val modules: List<GuideTemplateStepModule> = emptyList(),
)

@Serializable
data class GuideTemplateStepModule(
    val template: DocumentTemplateDefinition
)