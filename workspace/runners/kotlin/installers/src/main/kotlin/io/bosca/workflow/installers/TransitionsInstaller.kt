package io.bosca.workflow.installers

import io.bosca.api.Client
import io.bosca.installer.Installer
import io.bosca.workflow.models.TransitionDefinition
import io.bosca.workflow.ext.toInput
import io.bosca.workflow.yaml.YamlLoader
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.flow.toList
import java.io.File

class TransitionsInstaller : Installer {

    override suspend fun install(client: Client, directory: File) {
        val currentTransitions = client.workflows.getTransitions()
        val transitionsDir = File(directory, "transitions")

        if (!transitionsDir.exists()) {
            println("Transitions directory not found, skipping transition installation")
            return
        }

        val states = client.workflows.getStates()
        if (states.isEmpty()) {
            println("No states found. Please install states first.")
            return
        }

        val newTransitions = flow {
            transitionsDir.walk()
                .filter { it.isFile && it.extension == "yaml" }
                .forEach { file ->
                    try {
                        emit(YamlLoader.load(TransitionDefinition.serializer(), directory, file))
                    } catch (e: Exception) {
                        println("Error while loading transition from file ${file.name}: ${e.message}")
                    }
                }
        }.toList()

        for (transitionDef in newTransitions) {
            val transition = transitionDef.toInput(client)
            val existingTransition = currentTransitions.find {
                it.fromStateId == transition.fromStateId && it.toStateId == transition.toStateId
            }
            if (existingTransition != null) {
                client.workflows.editTransition(transition)
            } else {
                client.workflows.addTransition(transition)
            }
        }
    }
}