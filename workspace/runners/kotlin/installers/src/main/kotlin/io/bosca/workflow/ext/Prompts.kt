package io.bosca.workflow.ext

import io.bosca.graphql.type.PromptInput
import io.bosca.util.encodeToOptional
import io.bosca.util.toOptional
import io.bosca.workflow.models.PromptDefinition

fun PromptDefinition.toInput() = PromptInput(
    name = name,
    description = description,
    inputType = inputType,
    outputType = outputType,
    systemPrompt = systemPrompt,
    userPrompt = userPrompt,
    schema = schema?.encodeToOptional() ?: emptyMap<String, Any>().toOptional()
)