package io.bosca.installer

import io.bosca.api.Client
import java.io.File

interface Installer {

    suspend fun install(client: Client, directory: File)
}