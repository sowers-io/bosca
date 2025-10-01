package io.bosca.workflow.ext

import io.bosca.graphql.type.ProfileAttributeTypeInput
import io.bosca.workflow.models.ProfileAttributeTypeDefinition

fun ProfileAttributeTypeDefinition.toInput() = ProfileAttributeTypeInput(
    id = id,
    name = name,
    description = description,
    visibility = visibility,
    protected = false
)