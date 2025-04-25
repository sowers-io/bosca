package io.bosca.workflow.media.image

import io.bosca.api.Client
import io.bosca.graphql.fragment.Metadata
import io.bosca.graphql.fragment.MetadataRelationship
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.util.decode
import io.bosca.workflow.ActivityContext
import io.bosca.workflow.FullFailureException
import okhttp3.internal.format
import java.net.URLEncoder
import kotlin.uuid.ExperimentalUuidApi

class ImageRelationshipResizer(client: Client) : AbstractImageResizer(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Image Relationship Resizer",
            description = "Resizes images and creates variants based on configuration and using the images that are attached via the relationships",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    @OptIn(ExperimentalUuidApi::class)
    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val metadata = job.metadata?.metadata ?: throw FullFailureException("metadata missing")
        val relationships = client.metadata.getRelationships(metadata.id)
        if (relationships.isEmpty()) return
        val configuration = getConfiguration<ImageResizerConfiguration>(job)
        for (relationship in relationships) {
            if (relationship.metadata.metadataRelationshipMetadata.content.type.startsWith("image/")) {
                relationship.attributes?.let { attributes ->
                    val attr = attributes.decode<ImageAttributes>()
                    attr?.crop?.let {
                        val download =
                            client.metadata.getMetadataContentDownload(relationship.metadata.metadataRelationshipMetadata.id)
                                ?: error("failed to crop: missing supplementary content")
                        val url = URLEncoder.encode(download.urls.download.url, Charsets.UTF_8)
                        val supplementaryId = process(
                            context,
                            job,
                            relationship.metadata.metadataRelationshipMetadata.id,
                            url,
                            "jpeg",
                            ImageSize(
                                "${it.width}x${it.height}-${it.top}-${it.left}-cropped",
                                1,
                                it
                            )
                        )
                        process(context, job, configuration, metadata, relationship, supplementaryId, it)
                    }
                }
            }
        }
    }

    private suspend fun process(
        context: ActivityContext,
        job: WorkflowJob,
        configuration: ImageResizerConfiguration,
        metadata: Metadata,
        relationship: MetadataRelationship,
        supplementaryId: String,
        crop: Coordinates,
    ) {
        val content = client.metadata.getSupplementaryContentDownload(supplementaryId)
            ?: error("missing content")
        val url = URLEncoder.encode(content.urls.download.url, Charsets.UTF_8)
        val formats = mutableMapOf<String, Any>()
        ImageResizer.formats.forEach { format ->
            val key = "${crop.width}x${crop.height}-${crop.top}-${crop.left}-$format"
            if ((relationship.attributes as Map<*, *>).containsKey(key)) return@forEach
            val sizes = mutableMapOf<String, String>()
            for (size in configuration.sizes) {
                val newSize = size.copy(
                    name = "${size.name}-${crop.width}x${crop.height}-${crop.top}-${crop.left}-${size.ratio}-$format",
                    size = null
                )
                process(
                    context,
                    job,
                    relationship.metadata.metadataRelationshipMetadata.id,
                    url,
                    format,
                    newSize
                )
                sizes[size.name] = newSize.name
            }
            formats[format] = sizes
            formats[key] = true
        }
        if (formats.isEmpty()) return
        client.metadata.mergeRelationshipAttributes(
            metadata.id,
            relationship.metadata.metadataRelationshipMetadata.id,
            relationship.relationship,
            formats
        )
    }

    companion object {
        const val ID = "image.relationship.resizer"
    }
}