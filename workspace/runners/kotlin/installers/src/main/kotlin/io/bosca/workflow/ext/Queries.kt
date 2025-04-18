package io.bosca.workflow.ext

import io.bosca.graphql.type.*
import io.bosca.util.toOptional
import io.bosca.workflow.models.FindQueryDefinition

fun FindQueryDefinition.toInput(): FindQueryInput = FindQueryInput(
    collectionType = collectionType.toOptional(),
    attributes = attributes.map { FindAttributesInput(it.attributes.map { FindAttributeInput(it.key, it.value) }) },
    categoryIds = categoryIds.toOptional(),
    contentTypes = contentTypes.toOptional(),
    extensionFilter = extensionFilter.toOptional(),
)
