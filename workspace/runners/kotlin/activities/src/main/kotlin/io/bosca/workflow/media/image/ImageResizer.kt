package io.bosca.workflow.media.image

import com.apollographql.apollo.api.toUpload
import io.bosca.api.Client
import io.bosca.graphql.fragment.MetadataSupplementary
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.CollectionSupplementaryInput
import io.bosca.graphql.type.MetadataSupplementaryInput
import io.bosca.util.decode
import io.bosca.util.json
import io.bosca.util.toOptional
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import io.bosca.workflow.FullFailureException
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.serialization.Serializable
import java.io.File
import java.net.URL
import java.net.HttpURLConnection
import java.net.URLEncoder
import java.nio.file.Files
import kotlin.uuid.ExperimentalUuidApi
import kotlin.uuid.Uuid

@Serializable
data class Coordinates(
    val top: Int = 0,
    val left: Int = 0,
    val width: Int = 0,
    val height: Int = 0
) {
    val isEmpty: Boolean
        get() = width == 0 && height == 0 && top == 0 && left == 0
}

@Serializable
data class ImageAttributes(
    val crop: Coordinates? = null,
)

@Serializable
data class ImageSize(
    val name: String,
    val ratio: Int,
    val size: Coordinates? = null
)

@Serializable
data class ImageResizerConfiguration(
    val sizes: List<ImageSize>
)

abstract class AbstractImageResizer(client: Client) : Activity(client) {
    suspend fun process(
        context: ActivityContext,
        job: WorkflowJob,
        metadataId: String,
        url: String,
        format: String,
        size: ImageSize
    ): String {
        val imageProcessorUrl = System.getenv("IMAGE_PROCESSOR_URL") ?: "http://localhost:8003"
        val resized = if (size.size != null && !size.size.isEmpty) {
            "$imageProcessorUrl/image?u=$url&f=$format&w=${size.size.width}&h=${size.size.height}&l=${size.size.left}&t=${size.size.top}"
        } else {
            val ratio = size.ratio.toFloat() / 100f
            "$imageProcessorUrl/image?u=$url&f=$format&pw=${ratio}&ph=${ratio}"
        }
        val contentType = if (format == "jpeg") "image/jpg" else "image/webp"
        val file = downloadToFile(context, job.id, contentType, resized)
        val supplementaries = client.metadata.getSupplementary(metadataId)
        val supplementary = supplementaries
            .find { it.key == size.name }
            ?: client.metadata.addSupplementary(
                MetadataSupplementaryInput(
                    planId = job.planId.id,
                    name = size.name,
                    contentType = contentType,
                    key = size.name,
                    metadataId = metadataId
                )
            ) ?: error("failed to add supplementary")
        client.metadata.setSupplementaryContents(
            supplementary.id,
            file.toUpload(contentType),
        )
        return supplementary.id
    }
}

class ImageResizer(client: Client) : AbstractImageResizer(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Image Resizer",
            description = "Resizes images and creates variants based on configuration and relationships",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    @OptIn(ExperimentalUuidApi::class)
    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val metadata = job.metadata?.metadata ?: throw FullFailureException("metadata missing")
        if (!metadata.content.metadataContent.type.startsWith("image/")) return
        val configuration = getConfiguration<ImageResizerConfiguration>(job)
        val content = client.metadata.getMetadataContentDownload(metadata.id)
            ?: error("missing content")
        val url = URLEncoder.encode(content.urls.download.url, Charsets.UTF_8)
        formats.forEach { format ->
            for (size in configuration.sizes) {
                process(context, job, metadata.id, url, format, size)
            }
        }
    }

    companion object {
        const val ID = "image.resizer"
        val formats = arrayOf("jpeg", "webp")
    }
}