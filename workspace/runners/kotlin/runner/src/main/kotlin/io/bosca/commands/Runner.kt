package io.bosca.commands

import com.github.ajalt.clikt.command.SuspendingCliktCommand
import com.github.ajalt.clikt.parameters.options.default
import com.github.ajalt.clikt.parameters.options.option
import io.bosca.api.Client
import io.bosca.api.ClientProvider
import kotlin.system.exitProcess

class Runner(private val client: Client = ClientProvider.client) : SuspendingCliktCommand() {

    private val username by option().default(System.getenv("BOSCA_USERNAME") ?: "admin")
    private val password by option().default(System.getenv("BOSCA_PASSWORD") ?: "password")

    override suspend fun run() {
        try {
            client.security.login(username, password)
        } catch (e: Exception) {
            echo("Login failed.")
            echo(e.message)
            exitProcess(1)
        }
    }
}
