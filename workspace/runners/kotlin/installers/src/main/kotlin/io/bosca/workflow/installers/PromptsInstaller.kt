package io.bosca.workflow.installers

import io.bosca.api.Client
import io.bosca.installer.Installer
import io.bosca.workflow.models.PromptDefinition
import io.bosca.workflow.ext.toInput
import io.bosca.workflow.yaml.YamlLoader
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.flow.toList
import java.io.File

class PromptsInstaller: Installer {

    override suspend fun install(client: Client, directory: File) {
        val currentPrompts = client.workflows.getPrompts().associateBy { it.name }
        val newPrompts = flow {
            File(directory, "prompts").walk()
                .filter { it.isFile && it.extension == "yaml" }
                .forEach { file ->
                    try {
                        emit(YamlLoader.load(PromptDefinition.serializer(), directory, file).toInput())
                    } catch (e: Exception) {
                        println("Error while loading prompt from file ${file.name}: ${e.message}")
                    }
                }
        }.toList()
        for (prompt in newPrompts) {
            val current = currentPrompts[prompt.name]
            if (current != null) {
                client.workflows.editPrompt(current.id, prompt)
            } else {
                client.workflows.addPrompt(prompt)
            }
        }
    }
}
