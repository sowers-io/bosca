package io.bosca.commands.content

import com.github.ajalt.clikt.command.SuspendingCliktCommand
import io.bosca.api.Client
import io.bosca.api.ClientProvider

class Content(private val client: Client = ClientProvider.client) : SuspendingCliktCommand() {

    override suspend fun run() {
    }
}
