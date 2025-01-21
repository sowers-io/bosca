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
import kotlinx.coroutines.Job
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.joinAll
import kotlinx.coroutines.launch
import java.net.InetSocketAddress


class Run(
    private val client: Client = ClientProvider.client,
    private val registry: ActivityRegistry = ActivitiesInstaller(client)
) : SuspendingCliktCommand() {

    override suspend fun run() = coroutineScope {
        val runners = mutableListOf<JobRunner>()
        var server: HttpServer? = null

        Runtime.getRuntime().addShutdownHook(Thread {
            echo("\nShutting down...")
            for (runner in runners) {
                runner.shutdown()
            }
            client.security.keepTokenUpdated = false
            var shutdown = true
            do {
                for (runner in runners) {
                    if (runner.isShutdown()) {
                        shutdown = false
                        break
                    }
                }
            } while (!shutdown)
            try {
                server?.stop(0)
            } catch (ignore: Exception) {
            }
            echo("Shutdown.")
        })

        val jobs = mutableListOf<Job>()
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
            val job = launch {
                echo("running queue: $queue")
                val runner = EnterpriseActivityRegistryFactory.createRegistry(client)?.let { enterpriseRegistry ->
                    JobRunner(client, queue, max, object : ActivityRegistry {
                        override fun getActivity(id: String): Activity? {
                            return registry.getActivity(id) ?: enterpriseRegistry.getActivity(id)
                        }
                    })
                } ?: JobRunner(client, queue, max, registry)
                runner.run()
            }
            jobs.add(job)
        }

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

        jobs.joinAll()
    }
}