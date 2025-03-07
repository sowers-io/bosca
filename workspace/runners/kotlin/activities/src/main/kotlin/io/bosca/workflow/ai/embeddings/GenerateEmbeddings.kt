package io.bosca.workflow.ai.embeddings

import dev.langchain4j.data.document.Document
import dev.langchain4j.data.document.Metadata
import dev.langchain4j.data.document.splitter.*
import dev.langchain4j.model.embedding.onnx.HuggingFaceTokenizer
import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.ActivityParameterInput
import io.bosca.graphql.type.ActivityParameterType
import io.bosca.util.decode
import io.bosca.util.encodeToOptional
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import io.bosca.workflow.ext.*
import kotlinx.serialization.Serializable
import java.util.*

@Serializable
data class GenerateEmbeddingsConfiguration(
    val maxSegmentSizeInChars: Int = 512,
    val maxOverlapSizeInChars: Int = 100,
)

class GenerateEmbeddings(client: Client) : Activity(client) {

    override val id = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Generate Embeddings",
            description = "Generate Embeddings from Text",
            configuration = GenerateEmbeddingsConfiguration().encodeToOptional(),
            inputs = listOf(
                ActivityParameterInput(INPUT_NAME, ActivityParameterType.SUPPLEMENTARY)
            ),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val metadata = job.metadata?.metadata ?: error("metadata missing")
        val configuration =
            job.workflowActivity.workflowActivity.configuration.decode<GenerateEmbeddingsConfiguration>()
                ?: GenerateEmbeddingsConfiguration()

        val text = getInputSupplementaryText(context, job, INPUT_NAME)
        val (storage, _) = job.storageSystems.first()
        val model = client.workflows.getModel(storage.storageSystem.models.first().modelId) ?: error("model missing")
        val modelConfiguration = model.configuration.decode<ModelConfiguration>() ?: DefaultConfiguration()

        val embeddingStore = storage.storageSystem.toEmbeddingStore(client, modelConfiguration)
        try {
            val embeddingModel = model.toEmbeddingModel()

            val splitter = DocumentSplitters.recursive(
                configuration.maxSegmentSizeInChars,
                configuration.maxOverlapSizeInChars,
                HuggingFaceTokenizer()
            )
            val document = Document.from(
                text,
                Metadata.from(
                    mapOf(
                        "id" to UUID.fromString(metadata.id),
                        "version" to metadata.version
                    )
                ),
            )

            for (split in splitter.split(document)) {
                embeddingStore.add(embeddingModel.embed(split).content(), split)
            }
        } finally {
            embeddingStore.close()
        }
    }

    companion object {

        const val ID = "ai.embeddings.generate"
        const val INPUT_NAME = "supplementary"
    }
}