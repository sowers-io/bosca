package io.bosca.workflow.ext

import io.bosca.api.Client
import io.bosca.graphql.fragment.Category
import io.bosca.graphql.type.MetadataInput
import io.bosca.graphql.type.MetadataType
import io.bosca.util.encode
import io.bosca.util.toOptional
import io.bosca.workflow.models.CollectionDefinition
import io.bosca.workflow.models.CollectionTemplateDefinition
import io.bosca.workflow.models.DocumentTemplateDefinition
import io.bosca.workflow.models.GuideTemplateDefinition
import io.bosca.workflow.installers.EditorConfiguration

@Suppress("UNCHECKED_CAST")
fun CollectionTemplateDefinition.toCollectionTemplateInput(
    parentCollectionId: String,
    collection: CollectionDefinition,
    categories: Map<String, Category>
) = MetadataInput(
    name = collection.name + " Collection Template",
    attributes = (EditorConfiguration(
        editorType = "Template",
        templateType = collection.editorType,
    ).encode() as Map<String, Any> + collection.attributes).toOptional(),
    categoryIds = (collection.categories ?: emptyList()).map { categoryName ->
        categories[categoryName]?.id ?: error("Category `$categoryName` not found")
    }.toOptional(),
    collectionTemplate = collection.templates?.collection?.toInput().toOptional(),
    contentType = "bosca/v-collection-template",
    languageTag = "en",
    metadataType = MetadataType.STANDARD.toOptional(),
    parentCollectionId = parentCollectionId.toOptional(),
    slug = (collection.slug + "-collection-template").toOptional(),
)

@Suppress("UNCHECKED_CAST")
fun DocumentTemplateDefinition.toDocumentTemplateInput(
    parentCollectionId: String,
    collection: CollectionDefinition,
    categories: Map<String, Category>
) = MetadataInput(
    name = collection.name + " Document Template",
    attributes = (EditorConfiguration(
        editorType = "Template",
        templateType = collection.editorType,
    ).encode() as Map<String, Any> + collection.attributes).toOptional(),
    categoryIds = (collection.categories ?: emptyList()).map { categoryName ->
        categories[categoryName]?.id ?: error("Category `$categoryName` not found")
    }.toOptional(),
    documentTemplate = collection.templates?.guide?.guide?.template?.toInput().toOptional(),
    contentType = "bosca/v-document-template",
    languageTag = "en",
    metadataType = MetadataType.STANDARD.toOptional(),
    parentCollectionId = parentCollectionId.toOptional(),
    slug = (collection.slug + "-document-template").toOptional(),
)

@Suppress("UNCHECKED_CAST")
suspend fun GuideTemplateDefinition.toGuideTemplateInput(
    client: Client,
    parentCollectionId: String,
    collection: CollectionDefinition,
    categories: Map<String, Category>
): MetadataInput {
    val input = collection.templates?.guide?.toInput(client, parentCollectionId, collection, categories)
    return MetadataInput(
        name = collection.name + " Guide Template",
        attributes = (EditorConfiguration(
            editorType = "Template",
            templateType = collection.editorType,
        ).encode() as Map<String, Any> + collection.attributes).toOptional(),
        categoryIds = (collection.categories ?: emptyList()).map { categoryName ->
            categories[categoryName]?.id ?: error("Category `$categoryName` not found")
        }.toOptional(),
        guideTemplate = input?.first.toOptional(),
        documentTemplate = input?.second.toOptional(),
        contentType = "bosca/v-guide-template",
        languageTag = "en",
        metadataType = MetadataType.STANDARD.toOptional(),
        parentCollectionId = parentCollectionId.toOptional(),
        slug = (collection.slug + "-guide-template").toOptional(),
    )
}