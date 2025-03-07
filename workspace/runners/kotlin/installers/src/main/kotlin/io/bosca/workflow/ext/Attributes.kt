package io.bosca.workflow.ext

import io.bosca.graphql.type.AttributeType
import io.bosca.graphql.type.AttributeUiType
import io.bosca.graphql.type.TemplateAttributeInput
import io.bosca.graphql.type.TemplateWorkflowInput
import io.bosca.util.toOptional
import io.bosca.workflow.models.AttributeDefinition

fun AttributeDefinition.toInput() = TemplateAttributeInput(
    key = key,
    name = name,
    description = description,
    workflows = workflows.map { workflow ->
        TemplateWorkflowInput(
            autoRun = workflow.autoRun,
            workflowId = workflow.workflow
        )
    },
    type = AttributeType.valueOf(type),
    supplementaryKey = supplementaryKey.toOptional(),
    ui = AttributeUiType.valueOf(ui),
    list = list,
    configuration = configuration.toOptional()
)