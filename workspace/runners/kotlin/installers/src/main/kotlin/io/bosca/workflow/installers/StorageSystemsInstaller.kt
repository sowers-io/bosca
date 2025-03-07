package io.bosca.workflow.installers

import io.bosca.api.Client
import io.bosca.installer.Installer
import io.bosca.workflow.models.StorageSystemDefinition
import io.bosca.workflow.ext.toInput
import io.bosca.workflow.yaml.YamlLoader
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.flow.toList
import java.io.File

class StorageSystemsInstaller: Installer {
    override suspend fun install(client: Client, directory: File) {
        val currentSystems = client.workflows.getStorageSystems().associate {
            it.storageSystem.name to it.storageSystem.id
        }

        val modelIdsByName = client.workflows.getModels().associate { it.name to it.id }

        val newSystems = flow {
            File(directory, "storagesystems").walk()
                .filter { it.isFile && it.extension == "yaml" }
                .forEach { file ->
                    try {
                        emit(YamlLoader.load(StorageSystemDefinition.serializer(), directory, file).toInput(modelIdsByName))
                    } catch (e: Exception) {
                        println("Error while loading storage system from file ${file.name}: ${e.message}")
                    }
                }
        }.toList()

        for (system in newSystems) {
            val id = currentSystems[system.name]
            if (id != null) {
                client.workflows.editStorageSystem(id, system)
            } else {
                client.workflows.addStorageSystem(system)
            }
        }
    }
}
