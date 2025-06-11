package io.bosca.workflow.general

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import io.bosca.workflow.ActivityRegistry

class ActivityInstaller(client: Client) : Activity(client) {
    override val id = "activity.installer"

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Activity Installer",
            description = "Installs all activities",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(
        context: ActivityContext,
        job: WorkflowJob
    ) {
        ActivityRegistry.instance.install(client)
    }
}