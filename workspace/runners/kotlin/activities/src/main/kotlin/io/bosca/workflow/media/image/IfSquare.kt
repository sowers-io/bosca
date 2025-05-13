package io.bosca.workflow.media.image

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.util.json
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.serialization.Serializable
import java.net.URLEncoder

@Serializable
data class ImageMetadata(
    val format: String? = null,
    val size: Int? = null,
    val width: Int? = null,
    val height: Int? = null,
    val space: String? = null,
    val channels: Int? = null,
    val depth: String? = null,
    val density: Int? = null,
    val chromaSubsampling: String? = null
)

@Serializable
data class IfSquareConfiguration(
    val workflows: List<String> = emptyList(),
    val negate: Boolean = false,
)

@Serializable
data class IfSquareContext(val executed: Boolean = false)

class IfSquare(client: Client) : Activity(client) {
    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Run a child workflow if the image is square",
            description = "Run a child workflow if the image is square",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val ctx = getContext<IfSquareContext>(job)
        if (ctx.executed) return
        if (!(job.metadata?.metadata?.content?.metadataContent?.type ?: "").startsWith("image/")) return
        val cfg = getConfiguration<IfSquareConfiguration>(job)
        val download = client.metadata.getMetadataContentDownload(job.metadata?.metadata?.id ?: error("missing metadata id"))
                ?: error("failed to get metadata: missing content")
        val url = URLEncoder.encode(download.urls.download.url, Charsets.UTF_8)
        val imageProcessorUrl = "${System.getenv("IMAGE_PROCESSOR_URL") ?: "http://localhost:8003"}/metadata?u=$url"
        val file = downloadToFile(context, job.id, "text/json", imageProcessorUrl)
        val metadata = json.decodeFromString<ImageMetadata>(file.readText())
        val square = metadata.width == metadata.height
        if (square != cfg.negate) {
            client.workflows.enqueueChildWorkflows(
                cfg.workflows,
                job.id
            )
            setContext(job, IfSquareContext(executed = true))
        }
    }

    companion object {

        const val ID = "workflow.image.if.square"
    }
}