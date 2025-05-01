package io.bosca.workflow.ai.embeddings

import io.bosca.api.Client
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
data class QdrantConnectionConfiguration(
    val host: String,
    val port: Int,
    val apiKey: String? = null,
    val tls: Boolean = false,
)

@Serializable
data class QdrantConfiguration(
    val collectionName: String,
    val distance: Distance
) {

    suspend fun initialize(client: Client, model: ModelConfiguration) {
        withContext(Dispatchers.IO) {
            val cfg = client.configurations.get<QdrantConnectionConfiguration>("qdrant") ?: error("missing qdrant configuration")
            val builder = QdrantGrpcClient.newBuilder(cfg.host, cfg.port, cfg.tls)
            cfg.apiKey?.let { builder.withApiKey(it) }
            val qdrantClient = QdrantClient(builder.build())
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