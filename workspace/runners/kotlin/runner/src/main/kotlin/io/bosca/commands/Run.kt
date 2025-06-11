package io.bosca.commands

import com.github.ajalt.clikt.command.SuspendingCliktCommand
import com.sun.net.httpserver.HttpServer
import io.bosca.api.Client
import io.bosca.api.ClientProvider
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityRegistry
import io.bosca.workflow.EnterpriseActivityRegistryFactory
import io.bosca.workflow.JobRunner
import io.bosca.workflow.installers.ActivitiesInstaller
import kotlinx.coroutines.*
import java.net.InetSocketAddress

class Run(
    private val client: Client = ClientProvider.client,
    private val registry: ActivityRegistry = ActivitiesInstaller(client)
) : SuspendingCliktCommand() {

    @OptIn(DelicateCoroutinesApi::class)
    override suspend fun run() {
        val runners = mutableListOf<JobRunner>()
        var server: HttpServer? = null

        Thread {
            server = HttpServer.create(InetSocketAddress(9000), 0)
            server?.let { server ->
                server.createContext("/health") { exchange ->
                    exchange.sendResponseHeaders(200, 0)
                    exchange.responseBody.use { it.write("Healthy".toByteArray()) }
                }
                server.executor = null
                server.start()
            }
        }.start()

        val queues = System.getenv("BOSCA_QUEUES")?.trim() ?: ""
        for (queueConfig in queues.split(";")) {
            val queueParts = queueConfig.split(",")
            val queue = queueParts[0]
            if (queue.isBlank()) {
                continue
            }
            val max = queueParts.last().toIntOrNull() ?: 0
            if (max == 0) {
                echo("skipping queue: $queue")
                continue
            }
            echo("running queue: $queue")
            val runner = EnterpriseActivityRegistryFactory.createRegistry(client)?.let { enterpriseRegistry ->
                JobRunner(client, queue, max, object : ActivityRegistry {
                    override fun getActivity(id: String): Activity? {
                        return enterpriseRegistry.getActivity(id) ?: registry.getActivity(id)
                    }

                    override suspend fun install(client: Client) {
                        enterpriseRegistry.install(client)
                        registry.install(client)
                    }
                })
            } ?: JobRunner(client, queue, max, registry)
            runners.add(runner)
        }

        Runtime.getRuntime().addShutdownHook(Thread {
            echo("\nShutting down...")
            for (runner in runners) {
                runner.shutdown()
            }
            echo("\n...Shutdown Requested...")
            client.security.keepTokenUpdated = false
            var shutdown = true
            do {
                for (runner in runners) {
                    if (!runner.isShutdown()) {
                        println("runner not shutdown, waiting...")
                        shutdown = false
                        break
                    }
                }
            } while (!shutdown)
            println("...runners shutdown...")
            try {
                server?.stop(0)
            } catch (ignore: Exception) {}
            echo("Shutdown.")
        })

        for (runner in runners) {
            runner.run()
        }

        for (runner in runners) {
            if (runner.isShutdown()) {
                break
            }
            delay(1000)
        }
    }
}
