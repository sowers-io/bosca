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
): Pair<GuideTemplateInput, DocumentTemplateInput?> {
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
                )
            )
        }
        val stepMetadata = step.template?.toDocumentTemplateInput(
            parentCollectionId,
            collection,
            categories
        )
        val stepCurrent = stepMetadata?.let { client.get(it.slug.getOrThrow() ?: error("Missing slug"))?.metadata }
        val template = if (stepMetadata != null) {
            if (stepCurrent != null) {
                client.metadata.edit(stepCurrent.id, stepMetadata)
                stepCurrent
            } else {
                val id = client.metadata.add(stepMetadata) ?: error("Failed to add metadata")
                client.metadata.get(id) ?: error("Failed to get metadata")
            }
        } else {
            null
        }
        steps.add(
            GuideTemplateStepInput(
                templateMetadataId = template?.id.toOptional(),
                templateMetadataVersion = template?.version.toOptional(),
                modules = modules
            )
        )
    }
    return Pair(
        GuideTemplateInput(
            rrule = guide.rrule,
            type = GuideType.valueOf(guide.type),
            steps = steps
        ), guide.template?.toInput()
    )
}