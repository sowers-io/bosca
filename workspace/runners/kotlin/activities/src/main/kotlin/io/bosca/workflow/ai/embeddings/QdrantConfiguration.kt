package io.bosca.workflow.ai.embeddings

import io.bosca.api.Client
import io.bosca.api.KeyValue
import io.bosca.util.DefaultKeys
import io.bosca.workflow.ext.ModelConfiguration
import io.qdrant.client.QdrantClient
import io.qdrant.client.QdrantGrpcClient
import io.qdrant.client.grpc.Collections
import io.qdrant.client.grpc.Collections.VectorParams
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

enum class Distance {
    @SerialName("cosine")
    Cosine,

    @SerialName("euclid")
    Euclid,

    @SerialName("dot")
    Dot,

    @SerialName("manhattan")
    Manhattan
}

@Serializable
data class QdrantConfiguration(
    val collectionName: String,
    val distance: Distance
) {

    suspend fun initialize(client: Client, model: ModelConfiguration) {
        withContext(Dispatchers.IO) {
            val host = client.configurations.get<KeyValue>(DefaultKeys.QDRANT_HOST)?.value ?: "localhost"
            val port = (client.configurations.get<KeyValue>(DefaultKeys.QDRANT_PORT)?.value ?: "6334").toInt()
            val qdrantClient = QdrantClient(
                QdrantGrpcClient
                    .newBuilder(host, port, false)
                    .build()
            )

            try {
                qdrantClient.getCollectionInfoAsync(collectionName).get()
            } catch (e: Exception) {
                qdrantClient.createCollectionAsync(
                    collectionName,
                    VectorParams.newBuilder()
                        .setDistance(
                            when (distance) {
                                Distance.Cosine -> Collections.Distance.Cosine
                                Distance.Euclid -> Collections.Distance.Euclid
                                Distance.Dot -> Collections.Distance.Dot
                                Distance.Manhattan -> Collections.Distance.Manhattan
                            }
                        )
                        .setSize(model.dimension).build()
                ).get()
            } finally {
                qdrantClient.close()
            }
        }
    }
}