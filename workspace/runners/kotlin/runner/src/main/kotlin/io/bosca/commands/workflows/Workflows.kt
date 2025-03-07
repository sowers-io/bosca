package io.bosca.commands.workflows

import com.github.ajalt.clikt.command.SuspendingCliktCommand
import io.bosca.api.Client
import io.bosca.api.ClientProvider

class Workflows(private val client: Client = ClientProvider.client) : SuspendingCliktCommand() {

    override suspend fun run() {
    }
}
