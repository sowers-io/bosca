package io.bosca.workflow.installers

import io.bosca.api.Client
import io.bosca.api.KeyValue
import io.bosca.installer.Installer
import io.bosca.graphql.type.*
import io.bosca.util.encode
import io.bosca.workflow.models.Configurations
import io.bosca.workflow.yaml.YamlLoader
import java.io.File

class ConfigurationsInstaller : Installer {

    override suspend fun install(client: Client, directory: File) {
        val configurations = YamlLoader.load(Configurations.serializer(), directory, File(directory, "configurations.yaml"))
        for (configuration in configurations.configurations) {
            var value = configuration.value
            if (value is Map<*, *> && value.containsKey("env.variable")) {
                value = KeyValue(System.getenv(value["env.variable"].toString())).encode()
            }
            client.configurations.set(
                ConfigurationInput(
                    key = configuration.key,
                    description = configuration.description,
                    value = value,
                    permissions = emptyList()
                )
            )
        }
    }
}