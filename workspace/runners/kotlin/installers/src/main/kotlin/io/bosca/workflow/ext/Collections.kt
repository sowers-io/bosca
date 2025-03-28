package io.bosca.workflow.ext

import io.bosca.graphql.fragment.Category
import io.bosca.graphql.type.CollectionChildInput
import io.bosca.graphql.type.CollectionInput
import io.bosca.graphql.type.OrderingInput
import io.bosca.util.encode
import io.bosca.util.toOptional
import io.bosca.workflow.models.CollectionDefinition
import io.bosca.workflow.installers.EditorConfiguration

@Suppress("UNCHECKED_CAST")
fun CollectionDefinition.toInput(categories: Map<String, Category>): CollectionInput =
    CollectionInput(
        slug = slug.toOptional(),
        name = name,
        collectionType = type.toOptional(),
        attributes = (EditorConfiguration(
            editorType
        ).encode() as Map<String, Any> + attributes).toOptional(),
        categoryIds = (this.categories ?: emptyList()).map { categoryName ->
            categories[categoryName]?.id ?: error("Category `$categoryName` not found")
        }.toOptional(),
        collections = collections?.map { child ->
            CollectionChildInput(collection = child.toInput(categories))
        }.toOptional(),
        ordering = ordering?.map {
            OrderingInput(
                order = it.order,
                field = it.field.toOptional(),
                path = it.path.toOptional(),
                type = it.type.toOptional()
            )
        }.toOptional()
    )