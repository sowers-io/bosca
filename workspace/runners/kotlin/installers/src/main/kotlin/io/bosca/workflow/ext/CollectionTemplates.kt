package io.bosca.workflow.ext

import io.bosca.graphql.type.*
import io.bosca.util.toOptional
import io.bosca.workflow.models.CollectionTemplateDefinition

fun CollectionTemplateDefinition.toInput() = CollectionTemplateInput(
    configuration = collection.configuration.toOptional(),
    defaultAttributes = collection.defaultAttributes.toOptional(),
    filters = collection.filters?.let {
        CollectionTemplateFiltersInput(
            filters = it.filters.map { CollectionTemplateFilterInput(name = it.name, filter = it.filter) }
        )
    }.toOptional(),
    attributes = collection.attributes.map { it.toInput() },
    ordering = collection.ordering?.map {
        OrderingInput(
            order = it.order,
            field = it.field.toOptional(),
            location = it.location?.let { AttributeLocation.valueOf(it.name) }.toOptional(),
            path = it.path.toOptional(),
            type = it.type.toOptional()
        )
    }.toOptional()
)