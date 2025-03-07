package io.bosca.workflow.ext

import io.bosca.graphql.type.StorageSystemInput
import io.bosca.graphql.type.StorageSystemModelInput
import io.bosca.util.toOptional
import io.bosca.workflow.models.StorageSystemDefinition

fun StorageSystemDefinition.toInput(modelIdsByName: Map<String, String>): StorageSystemInput {
    return StorageSystemInput(
        configuration = configuration.toOptional(),
        description = description,
        models = models.map { model ->
            StorageSystemModelInput(
                configuration = model.configuration,
                modelId = modelIdsByName[model.model] ?: error("Model ${model.model} not found")
            )
        },
        name = name,
        type = type
    )
}
