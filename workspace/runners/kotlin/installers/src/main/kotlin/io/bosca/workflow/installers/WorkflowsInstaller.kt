package io.bosca.workflow.installers

import io.bosca.api.Client
import io.bosca.installer.Installer
import io.bosca.workflow.models.WorkflowDefinition.Companion.serializer
import io.bosca.workflow.ext.toInput
import io.bosca.workflow.yaml.YamlLoader
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.flow.toList
import java.io.File

class WorkflowsInstaller : Installer {

    override suspend fun install(client: Client, directory: File) {
        val existingWorkflows = client.workflows.getWorkflows().associateBy { it.id }
        val newWorkflows = flow {
            File(directory, "workflows").walk()
                .filter { it.isFile && it.extension == "yaml" }
                .forEach { file ->
                    emit(YamlLoader.load(serializer(), directory, file).toInput(client))
                }
        }.toList()
        for (workflow in newWorkflows) {
            if (existingWorkflows.containsKey(workflow.id)) {
                client.workflows.edit(workflow)
            } else {
                client.workflows.add(workflow)
            }
        }
        client.clearCache()
        client.security.refreshToken()
    }
}
