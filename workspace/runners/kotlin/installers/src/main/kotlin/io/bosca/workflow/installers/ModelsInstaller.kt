package io.bosca.workflow.installers

import io.bosca.api.Client
import io.bosca.installer.Installer
import io.bosca.workflow.models.ModelDefinition
import io.bosca.workflow.ext.toInput
import io.bosca.workflow.yaml.YamlLoader
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.flow.toList
import java.io.File

class ModelsInstaller: Installer {
    override suspend fun install(client: Client, directory: File) {
        val currentModels = client.workflows.getModels().associateBy { it.name }
        val newModels = flow {
            File(directory, "models").walk()
                .filter { it.isFile && it.extension == "yaml" }
                .forEach { file ->
                    try {
                        emit(YamlLoader.load(ModelDefinition.serializer(), directory, file).toInput())
                    } catch (e: Exception) {
                        println("Error while loading model from file ${file.name}: ${e.message}")
                    }
                }
        }.toList()

        for (model in newModels) {
            val current = currentModels[model.name]
            if (current != null) {
                client.workflows.editModel(current.id, model)
            } else {
                client.workflows.addModel(model)
            }
        }
    }
}
