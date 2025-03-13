package io.bosca.workflow.ext

import dev.langchain4j.data.segment.TextSegment
import dev.langchain4j.store.embedding.EmbeddingStore
import dev.langchain4j.store.embedding.qdrant.QdrantEmbeddingStore
import io.bosca.api.Client
import io.bosca.graphql.fragment.StorageSystem
import io.bosca.graphql.type.StorageSystemType
import io.bosca.util.decode
import io.bosca.workflow.ai.embeddings.QdrantConfiguration
import io.bosca.workflow.ai.embeddings.QdrantConnectionConfiguration

suspend fun StorageSystem.toEmbeddingStore(client: Client, model: ModelConfiguration): EmbeddingStore<TextSegment> {
    if (type != StorageSystemType.VECTOR) throw Exception("Unsupported storage system type: $type")
    val connection = client.configurations.get<QdrantConnectionConfiguration>("qdrant") ?: error("missing qdrant configuration")
    val cfg = configuration.decode<QdrantConfiguration>()
    cfg?.initialize(client, model)
    return QdrantEmbeddingStore.builder()
        .host(connection.host)
        .port(connection.port)
        .useTls(connection.tls)
        .collectionName(cfg?.collectionName ?: "embeddings")
        .build()
}

suspend fun EmbeddingStore<TextSegment>.close() {
    if (this is QdrantEmbeddingStore) {
        this.close()
    }
}