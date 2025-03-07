package io.bosca.workflow.installers

import io.bosca.api.Client
import io.bosca.graphql.type.TraitInput
import io.bosca.installer.Installer
import io.bosca.workflow.models.Traits
import io.bosca.workflow.yaml.YamlLoader
import java.io.File

class TraitsInstaller : Installer {

    override suspend fun install(client: Client, directory: File) {
        val current = client.workflows.getTraits().mapTo(mutableSetOf()) { it.id }
        val traits = YamlLoader.load(Traits.serializer(), directory, File(directory, "traits.yaml"))
        for (trait in traits.traits) {
            val input = TraitInput(
                id = trait.id,
                name = trait.name,
                description = trait.description,
                workflowIds = trait.workflowIds,
                contentTypes = trait.contentTypes
            )
            if (current.contains(trait.id)) {
                client.workflows.editTrait(input)
            } else {
                client.workflows.addTrait(input)
            }
        }
    }
}
