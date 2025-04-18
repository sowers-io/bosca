package io.bosca.workflow.ext

import io.bosca.api.Client
import io.bosca.graphql.fragment.Category
import io.bosca.graphql.type.AttributeLocation
import io.bosca.graphql.type.CollectionChildInput
import io.bosca.graphql.type.CollectionInput
import io.bosca.graphql.type.FindAttributeInput
import io.bosca.graphql.type.OrderingInput
import io.bosca.util.encode
import io.bosca.util.toOptional
import io.bosca.workflow.models.CollectionDefinition
import io.bosca.workflow.installers.EditorConfiguration

@Suppress("UNCHECKED_CAST")
suspend fun CollectionDefinition.toInput(
    client: Client,
    categories: Map<String, Category>
): CollectionInput {
    var templateId: String? = null
    var templateVersion: Int? = null
    if (templates?.query != null) {
        val metadata = client.metadata.findMetadata(
            templates.query.attributes.flatMap {
                it.attributes.map {
                    FindAttributeInput(
                        it.key,
                        it.value
                    )
                }
            },
            offset = 0,
            limit = 1
        ).firstOrNull() ?: error("Failed to get metadata: ${templates.query}")
        templateId = metadata.id
        templateVersion = metadata.version
    }
    return CollectionInput(
        slug = slug.toOptional(),
        name = name,
        collectionType = type.toOptional(),
        attributes = (EditorConfiguration(
            editorType
        ).encode() as Map<String, Any> + attributes).toOptional(),
        categoryIds = (this.categories ?: emptyList()).map { categoryName ->
            categories[categoryName]?.id ?: error("Category `$categoryName` not found")
        }.toOptional(),
        templateMetadataId = templateId.toOptional(),
        templateMetadataVersion = templateVersion.toOptional(),
        collections = collections?.map { child ->
            CollectionChildInput(collection = child.toInput(client, categories))
        }.toOptional(),
        ordering = ordering?.map {
            OrderingInput(
                order = it.order,
                field = it.field.toOptional(),
                path = it.path.toOptional(),
                type = it.type.toOptional(),
                location = it.location?.let { AttributeLocation.valueOf(it.name) }.toOptional()
            )
        }.toOptional()
    )
}