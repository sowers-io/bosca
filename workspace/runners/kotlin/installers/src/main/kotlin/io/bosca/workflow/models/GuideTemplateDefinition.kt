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
    val defaultAttributes: Map<String, String> = emptyMap(),
    val type: String = "",
    val rrule: String = "",
    val steps: List<GuideTemplateStep> = emptyList(),
    val attributes: List<AttributeDefinition> = emptyList()
)

@Serializable
data class GuideTemplateStep(
    val title: String = "",
    val description: String = "",
    val modules: List<GuideTemplateStepModule> = emptyList(),
    val attributes: List<AttributeDefinition> = emptyList()
)

@Serializable
data class GuideTemplateStepModule(
    val template: DocumentTemplateDefinition,
    val configuration: Map<String, String> = emptyMap()
)