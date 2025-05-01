package io.bosca.workflow.installers

import io.bosca.api.Client
import io.bosca.graphql.type.CategoryInput
import io.bosca.installer.Installer
import io.bosca.workflow.models.Categories
import io.bosca.yaml.YamlLoader
import java.io.File

class CategoriesInstaller(val client: Client) : Installer {

    override suspend fun install(client: Client, directory: File) {
        val categories = YamlLoader.load(Categories.serializer(), directory, File(directory, "categories.yaml"))
        val currentCategories = client.categories.getAll().associateBy { it.name }
        for (category in categories.categories) {
            currentCategories[category]?.let {
                client.categories.edit(it.id, CategoryInput(category))
            } ?: client.categories.add(CategoryInput(category))
        }
    }
}
