import com.apollographql.apollo.api.toUpload
import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.MetadataSupplementaryInput
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.io.File
import java.nio.file.Files
import kotlin.uuid.ExperimentalUuidApi
import kotlin.uuid.Uuid

class ThumbnailExtractor(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Thumbnail Extractor",
            description = "Extracts thumbnails from a video",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    @OptIn(ExperimentalUuidApi::class)
    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val metadata = job.metadata?.metadata ?: error("metadata missing")
        val file = getContentFile(context, job)
        withContext(Dispatchers.IO) {
            val outputDirectory = Files.createTempDirectory("thumbnails-${Uuid.random().toHexString()}").toFile()
            context.addFile(outputDirectory)
            extract(file, outputDirectory)
            val thumbnails = outputDirectory.listFiles() ?: return@withContext
            for ((index, thumbnail) in thumbnails.withIndex()) {
                if (index > 10) break
                val supplementary = client.metadata.addSupplementary(
                    MetadataSupplementaryInput(
                        contentType = "image/jpeg",
                        key = "thumbnail-$index",
                        metadataId = metadata.id,
                        name = "Thumbnail #$index",
                    )
                ) ?: error("failed to add supplementary")
                client.metadata.setSupplementaryContents(metadata.id, supplementary.key, thumbnail.toUpload("image/jpeg"))
            }
        }
    }

    private fun extract(inputFile: File, outputDirectory: File) {
        val ffmpegCmd = listOf(
            "ffmpeg",
            "-hide_banner",
            "-loglevel", "error",
            "-y",
            "-i", inputFile.absolutePath,
            "-vf", "select='eq(pict_type,I)',scale=800:-1",
            "-vsync", "vfr",
            "-q:v", "2",
            "${outputDirectory.absolutePath}/thumbnail%04d.jpg"
        )
        val process = ProcessBuilder(ffmpegCmd)
            .redirectError(ProcessBuilder.Redirect.INHERIT)
            .redirectOutput(ProcessBuilder.Redirect.INHERIT)
            .start()
        val exitCode = process.waitFor()
        if (exitCode != 0) {
            throw RuntimeException("ffmpeg process failed with exit code $exitCode")
        }
    }

    companion object {

        const val ID = "video.thumbnail.extractor"
    }
}