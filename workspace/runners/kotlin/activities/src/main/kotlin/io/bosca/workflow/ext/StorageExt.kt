package io.bosca.workflow.ext

import dev.langchain4j.data.segment.TextSegment
import dev.langchain4j.store.embedding.EmbeddingStore
import dev.langchain4j.store.embedding.qdrant.QdrantEmbeddingStore
import io.bosca.api.Client
import io.bosca.api.KeyValue
import io.bosca.graphql.fragment.StorageSystem
import io.bosca.graphql.type.StorageSystemType
import io.bosca.util.DefaultKeys
import io.bosca.util.decode
import io.bosca.workflow.ai.embeddings.QdrantConfiguration

suspend fun StorageSystem.toEmbeddingStore(client: Client, model: ModelConfiguration): EmbeddingStore<TextSegment> {
    if (type != StorageSystemType.VECTOR) throw Exception("Unsupported storage system type: $type")
    val qdrantConfiguration = configuration.decode<QdrantConfiguration>()
    qdrantConfiguration?.initialize(client, model)
    val host = client.configurations.get<KeyValue>(DefaultKeys.QDRANT_HOST)?.value ?: "localhost"
    val port = (client.configurations.get<KeyValue>(DefaultKeys.QDRANT_PORT)?.value ?: "6334").toInt()
    return QdrantEmbeddingStore.builder()
            .host(host)
            .port(port)
            .collectionName(qdrantConfiguration?.collectionName ?: "embeddings")
            .build()
}

suspend fun EmbeddingStore<TextSegment>.close() {
    if (this is QdrantEmbeddingStore) {
        this.close()
    }
}