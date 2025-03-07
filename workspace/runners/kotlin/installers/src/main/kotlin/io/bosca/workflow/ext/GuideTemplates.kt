package io.bosca.workflow.ext

import io.bosca.api.Client
import io.bosca.graphql.fragment.Category
import io.bosca.graphql.type.*
import io.bosca.util.toOptional
import io.bosca.workflow.models.CollectionDefinition
import io.bosca.workflow.models.GuideTemplateDefinition

suspend fun GuideTemplateDefinition.toInput(
    client: Client,
    parentCollectionId: String,
    collection: CollectionDefinition,
    categories: Map<String, Category>
): GuideTemplateInput {
    val steps = mutableListOf<GuideTemplateStepInput>()
    for (step in guide.steps) {
        val modules = mutableListOf<GuideTemplateStepModuleInput>()
        for (module in step.modules) {
            val metadata = module.template.toDocumentTemplateInput(
                parentCollectionId,
                collection,
                categories
            )
            val current = client.get(metadata.slug.getOrThrow() ?: error("Missing slug"))?.metadata
            val template = if (current != null) {
                client.metadata.edit(current.id, metadata)
                current
            } else {
                val id = client.metadata.add(metadata) ?: error("Failed to add metadata")
                client.metadata.get(id) ?: error("Failed to get metadata")
            }
            modules.add(
                GuideTemplateStepModuleInput(
                    templateMetadataId = template.id,
                    templateMetadataVersion = template.version,
                    configuration = module.configuration.toOptional()
                )
            )
        }
        steps.add(
            GuideTemplateStepInput(
                attributes = step.attributes.map { it.toInput() },
                modules = modules
            )
        )
    }
    return GuideTemplateInput(
        attributes = guide.attributes.map { it.toInput() },
        defaultAttributes = guide.defaultAttributes.toOptional(),
        rrule = guide.rrule,
        type = GuideType.valueOf(guide.type),
        steps = steps
    )
}