package io.bosca.workflow.installers

import io.bosca.api.Client
import io.bosca.installer.Installer
import io.bosca.workflow.models.StateDefinition
import io.bosca.workflow.ext.toInput
import io.bosca.workflow.yaml.YamlLoader
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.flow.toList
import java.io.File

class StatesInstaller : Installer {

    override suspend fun install(client: Client, directory: File) {
        val currentStates = client.workflows.getStates().associateBy { it.id }
        val statesDir = File(directory, "states")

        if (!statesDir.exists()) {
            println("States directory not found, skipping state installation")
            return
        }

        val newStates = flow {
            statesDir.walk()
                .filter { it.isFile && it.extension == "yaml" }
                .forEach { file ->
                    try {
                        emit(YamlLoader.load(StateDefinition.serializer(), directory, file).toInput())
                    } catch (e: Exception) {
                        println("Error while loading state from file ${file.name}: ${e.message}")
                    }
                }
        }.toList()

        for (state in newStates) {
            val current = currentStates[state.id]
            if (current != null) {
                client.workflows.editState(state)
            } else {
                client.workflows.addState(state)
            }
        }
    }
}