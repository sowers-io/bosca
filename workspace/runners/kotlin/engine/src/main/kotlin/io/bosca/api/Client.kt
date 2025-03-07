package io.bosca.api

import io.bosca.graphql.GetSlugQuery
import io.bosca.graphql.fragment.Metadata
import io.bosca.graphql.fragment.Collection
import io.bosca.graphql.fragment.ProfileIdName

data class SlugItem(
    val metadata: Metadata? = null,
    val collection: Collection? = null,
    val profile: ProfileIdName? = null
)

class Client(val network: NetworkClient) {

    val security = Security(network)
    val collections = ContentCollections(network)
    val metadata = ContentMetadata(network)
    val categories = ContentCategories(network)
    val configurations = Configurations(network)
    val workflows = Workflows(network)
    val search = Search(network)
    val profiles = Profiles(network)
    val listeners = Listeners(network)
    val runPod = RunPod(this)
    val files = Files(network)

    suspend fun get(slug: String): SlugItem? {
        val response = network.graphql.query(GetSlugQuery(slug)).execute()
        response.validate()
        return response.data?.content?.slug?.let {
            SlugItem(
                it.onMetadata?.metadata,
                it.onCollection?.collection,
                it.onProfile?.profileIdName
            )
        }
    }
}

object ClientProvider {

    val client by lazy { Client(NetworkClientProvider.client) }
}