package io.bosca.workflow.storage

import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch

class UpdateStorage(client: io.bosca.api.Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Update Storage Data",
            description = "Update Storage Data",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) = coroutineScope {
        val metadatas = launch {
            var offset = 0
            do {
                val metadatas = client.metadata.getAll(offset, 100)
                if (metadatas.isEmpty()) break
                for (metadata in metadatas) {
                    client.workflows.enqueueMetadataWorkflow(
                        "metadata.update.storage",
                        metadata.id,
                        metadata.version
                    )
                }
                offset += 100
            } while (true)
        }

        val collections = launch {
            var offset = 0
            do {
                val collections = client.collections.getAll(offset, 100)
                if (collections.isEmpty()) break
                for (collection in collections) {
                    client.workflows.enqueueCollectionWorkflow(
                        "collection.update.storage",
                        collection.id
                    )
                }
                offset += 100
            } while (true)
        }

//        val profiles = launch {
//            var offset = 0
//            do {
//                val profiles = client.profiles.getAll(offset, 100)
//                if (profiles.isEmpty()) break
//                for ((profile, _) in profiles) {
//                    client.workflows.enqueueProfileWorkflow(
//                        "profile.update.storage",
//                        profile.id
//                    )
//                }
//                offset += 100
//            } while (true)
//        }

//        arrayOf(metadatas, collections, profiles).forEach { it.join() }
        arrayOf(metadatas, collections).forEach { it.join() }
    }

    companion object {
        const val ID = "storage.update.all"
    }
}