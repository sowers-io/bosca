package io.bosca.workflow.ext

import io.bosca.graphql.type.*
import io.bosca.util.toOptional
import io.bosca.workflow.models.CollectionTemplateDefinition

fun CollectionTemplateDefinition.toInput() = CollectionTemplateInput(
    configuration = collection.configuration.toOptional(),
    defaultAttributes = collection.defaultAttributes.toOptional(),
    collectionFilter = collection.collectionsFilter?.toInput().toOptional(),
    metadataFilter = collection.metadataFilter?.toInput().toOptional(),
    attributes = collection.attributes.map { it.toInput() }
)