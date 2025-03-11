package io.bosca.workflow.ext

import io.bosca.graphql.type.WorkflowStateInput
import io.bosca.graphql.type.WorkflowStateType
import io.bosca.util.toOptional
import io.bosca.workflow.models.StateDefinition

fun StateDefinition.toInput(): WorkflowStateInput {
    return WorkflowStateInput(
        id = id,
        name = name,
        description = description,
        type = WorkflowStateType.valueOf(type),
        configuration = configuration ?: emptyMap<String, Any>(),
        workflowId = workflowId.toOptional(),
        exitWorkflowId = exitWorkflowId.toOptional(),
        entryWorkflowId = entryWorkflowId.toOptional(),
    )
}