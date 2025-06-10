package io.bosca.workflow.ext

import com.apollographql.apollo.api.Optional
import io.bosca.api.Client
import io.bosca.graphql.type.*
import io.bosca.util.toOptional
import io.bosca.workflow.models.WorkflowDefinition

suspend fun WorkflowDefinition.toInput(client: Client): WorkflowInput {
    var executionGroup = 0
    return WorkflowInput(
        id = id,
        name = name,
        description = description,
        queue = queue,
        configuration = configuration ?: emptyMap<String, Any>(),
        activities = activities.map { activity ->
            executionGroup = activity.executionGroup ?: (executionGroup + 1)
            WorkflowActivityInput(
                queue = activity.queue,
                activityId = activity.activity,
                description = activity.description ?: "",
                executionGroup = executionGroup,
                configuration = activity.configuration.toOptional(),
                inputs = activity.inputs.map { input ->
                    WorkflowActivityParameterInput(input.name, input.supplementary)
                },
                outputs = activity.outputs.map { output ->
                    WorkflowActivityParameterInput(output.name, output.supplementary)
                },
                models = activity.models.map { model ->
                    WorkflowActivityModelInput(
                        modelId = client.workflows.getModels().first { it.name == model.name }.id
                    )
                },
                prompts = activity.prompts.map { prompt ->
                    WorkflowActivityPromptInput(
                        promptId = client.workflows.getPrompts().first { it.name == prompt.name }.id
                    )
                },
                storageSystems = activity.storageSystems.map { storage ->
                    WorkflowActivityStorageSystemInput(
                        configuration = Optional.Absent,
                        systemId = client.workflows.getStorageSystems().first {
                            it.name == storage.name
                        }.id
                    )
                }
            )
        }
    )
}