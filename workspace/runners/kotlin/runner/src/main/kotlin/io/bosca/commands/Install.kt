package io.bosca.commands

import com.github.ajalt.clikt.command.SuspendingCliktCommand
import com.github.ajalt.clikt.parameters.options.option
import com.github.ajalt.clikt.parameters.options.required
import io.bosca.api.Client
import io.bosca.api.ClientProvider
import io.bosca.installer.InstallerExecutor
import io.bosca.workflow.InstallerExecutorFactory
import java.io.File
import kotlin.system.exitProcess

class Install(
    private val client: Client = ClientProvider.client,
    private val installerExecutor: InstallerExecutor = InstallerExecutorFactory.create(client),
) : SuspendingCliktCommand() {

    private val directory by option().required()

    override suspend fun run() {
        echo("Installing...")
        val file = File(directory)
        installerExecutor.execute(client, file)
        echo("Installed.")
        exitProcess(0)
    }
}