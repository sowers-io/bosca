package io.bosca.workflow.media.image

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.util.decode
import io.bosca.workflow.ActivityContext
import io.bosca.workflow.FullFailureException
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
                val attributes = relationship.attributes
                if (attributes != null) {
                    val attr = attributes.decode<ImageAttributes>()
                    attr?.crop?.let {
                        val download = client.metadata.getMetadataContentDownload(relationship.metadata.metadataRelationshipMetadata.id)
                            ?: error("failed to crop: missing supplementary content")
                        var url = URLEncoder.encode(download.urls.download.url, Charsets.UTF_8)
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
                        val content = client.metadata.getSupplementaryContentDownload(supplementaryId)
                            ?: error("missing content")
                        url = URLEncoder.encode(content.urls.download.url, Charsets.UTF_8)
                        ImageResizer.formats.forEach { format ->
                            for (size in configuration.sizes) {
                                process(
                                    context,
                                    job,
                                    relationship.metadata.metadataRelationshipMetadata.id,
                                    url,
                                    format,
                                    size.copy(name = "${size.name}-${it.width}x${it.height}-${it.top}-${it.left}-$format")
                                )
                            }
                        }
                    }
                }
            }
        }
    }

    companion object {
        const val ID = "image.relationship.resizer"
    }
}