package io.bosca.api

import com.apollographql.apollo.api.Optional
import io.bosca.graphql.SearchQuery

class Search(network: NetworkClient) : Api(network) {

    suspend fun search(query: String, filter: String?, storageSystemId: String): List<SearchQuery.Document> {
        val response = network.graphql.query(
            SearchQuery(
                query = query,
                filter = filter?.let { Optional.present(it) } ?: Optional.Absent,
                storageSystemId = storageSystemId
            )
        ).execute()
        response.validate()
        return response.data?.search?.documents ?: emptyList()
    }
}