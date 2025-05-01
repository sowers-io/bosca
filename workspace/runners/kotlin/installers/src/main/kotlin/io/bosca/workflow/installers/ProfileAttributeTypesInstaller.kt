package io.bosca.workflow.installers

import io.bosca.api.Client
import io.bosca.installer.Installer
import io.bosca.workflow.models.ProfileAttributeTypeDefinition
import io.bosca.workflow.ext.toInput
import io.bosca.yaml.YamlLoader
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.flow.toList
import java.io.File

class ProfileAttributeTypesInstaller : Installer {
    override suspend fun install(client: Client, directory: File) {
        val currentTypes = client.profiles.getAttributeTypes().associateBy { it.id }
        val newTypes = flow {
            File(directory, "profileattributes").walk()
                .filter { it.isFile && it.extension == "yaml" }
                .forEach { file ->
                    try {
                        emit(YamlLoader.load(ProfileAttributeTypeDefinition.serializer(), directory, file).toInput())
                    } catch (e: Exception) {
                        println("Error while loading profile attribute type from file ${file.name}: ${e.message}")
                    }
                }
        }.toList()

        for (type in newTypes) {
            if (!currentTypes.containsKey(type.id)) {
                client.profiles.addAttributeType(type)
            } else {
                client.profiles.editAttributeType(type)
            }
        }
    }
}
