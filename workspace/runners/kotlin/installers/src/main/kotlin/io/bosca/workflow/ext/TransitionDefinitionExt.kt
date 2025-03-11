package io.bosca.workflow.ext

import io.bosca.api.Client
import io.bosca.graphql.type.TransitionInput
import io.bosca.workflow.models.TransitionDefinition

suspend fun TransitionDefinition.toInput(client: Client): TransitionInput {
    val states = client.workflows.getStates()
    val fromStateId = states.find { it.name == fromState }?.id
        ?: throw IllegalArgumentException("Could not find state with name: $fromState")
    val toStateId = states.find { it.name == toState }?.id
        ?: throw IllegalArgumentException("Could not find state with name: $toState")
    
    return TransitionInput(
        fromStateId = fromStateId,
        toStateId = toStateId,
        description = description
    )
}