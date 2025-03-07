package io.bosca.installer

import io.bosca.api.Client
import java.io.File

class InstallerExecutor(val installers: List<Installer>) {

    suspend fun execute(client: Client, directory: File) {
        for (installer in installers) {
            installer.install(client, directory)
        }
    }
}