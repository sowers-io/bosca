package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.serialization.Serializable

@Serializable
data class PublishCollectionConfiguration(
    val public: Boolean = true,
    val publicList: Boolean = true,
)

class PublishCollection(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Publish Collection",
            description = "Publish Collection",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val collection = job.collection?.collection ?: return
        val configuration = getConfiguration<PublishCollectionConfiguration>(job)
        if (!collection.public && configuration.public) {
            client.collections.setPublic(collection.id, true)
        }
        if (!collection.publicList && configuration.publicList) {
            client.collections.setPublicList(collection.id, true)
        }
        if (!collection.locked) {
            client.collections.setLocked(collection.id, true)
        }
    }

    companion object {
        const val ID = "collection.publish"
    }
}