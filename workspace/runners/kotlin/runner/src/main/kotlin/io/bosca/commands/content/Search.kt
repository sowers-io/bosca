package io.bosca.commands.content

import com.github.ajalt.clikt.command.SuspendingCliktCommand
import com.github.ajalt.clikt.parameters.options.option
import com.github.ajalt.clikt.parameters.options.required
import io.bosca.api.Client
import io.bosca.api.ClientProvider
import kotlin.system.exitProcess

class Search(private val client: Client = ClientProvider.client) : SuspendingCliktCommand() {

    private val query by option().required()
    private val filter by option()

    override suspend fun run() {
        val storageSystem = client
            .workflows
            .getStorageSystems()
            .firstOrNull { it.storageSystem.name == "Default Search" }
            ?.storageSystem ?: error("missing storage system")
        val results = client.search.search(
            query = query,
            filter = filter,
            storageSystemId = storageSystem.id
        )
        echo(results.joinToString("\n"))
        exitProcess(0)
    }
}
