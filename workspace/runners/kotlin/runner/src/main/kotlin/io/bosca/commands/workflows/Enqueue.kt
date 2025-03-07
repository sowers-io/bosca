package io.bosca.commands.workflows

import com.github.ajalt.clikt.command.SuspendingCliktCommand
import com.github.ajalt.clikt.parameters.options.option
import com.github.ajalt.clikt.parameters.options.required
import io.bosca.api.Client
import io.bosca.api.ClientProvider
import kotlin.system.exitProcess

class Enqueue(private val client: Client = ClientProvider.client) : SuspendingCliktCommand() {

    private val metadataId by option()
    private val collectionId by option()
    private val workflowId by option().required()

    override suspend fun run() {
        metadataId?.let {
            val metadata = client.metadata.get(it)
            if (metadata == null) {
                echo("missing metadata")
                exitProcess(1)
            }
            client.workflows.enqueueMetadataWorkflow(workflowId, metadata.id, metadata.version)
        }
        collectionId?.let {
            client.workflows.enqueueCollectionWorkflow(workflowId, it)
        }
        exitProcess(0)
    }
}
