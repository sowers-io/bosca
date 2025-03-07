package io.bosca.workflow.ext

import io.bosca.graphql.type.ModelInput
import io.bosca.workflow.models.ModelDefinition

fun ModelDefinition.toInput() = ModelInput(
    configuration = configuration ?: emptyMap<String, Any>(),
    description = description,
    name = name,
    type = type
)
