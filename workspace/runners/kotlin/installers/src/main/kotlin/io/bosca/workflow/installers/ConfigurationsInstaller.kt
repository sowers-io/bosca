package io.bosca.workflow.installers

import io.bosca.api.Client
import io.bosca.installer.Installer
import io.bosca.graphql.type.*
import io.bosca.workflow.models.Configurations
import io.bosca.workflow.yaml.YamlLoader
import java.io.File

class ConfigurationsInstaller : Installer {

    private fun update(value: Any): Any =
        when (value) {
            is Map<*, *> -> value.mapValues {
                when (it.value) {
                    is Map<*, *> -> {
                        val map = it.value as Map<*, *>
                        if (map.containsKey("env.variable")) {
                            System.getenv(map["env.variable"].toString())
                        } else {
                            update(map)
                        }
                    }

                    else -> update(it.value as Any)
                }
            }

            is List<*> -> value.map { update(it as Any) }
            else -> value
        }

    override suspend fun install(client: Client, directory: File) {
        val configurations = YamlLoader.load(Configurations.serializer(), directory, File(directory, "configurations.yaml"))
        for (configuration in configurations.configurations) {
            client.configurations.set(
                ConfigurationInput(
                    key = configuration.key,
                    description = configuration.description,
                    public = configuration.public,
                    value = update(configuration.value),
                    permissions = emptyList()
                )
            )
        }
    }
}