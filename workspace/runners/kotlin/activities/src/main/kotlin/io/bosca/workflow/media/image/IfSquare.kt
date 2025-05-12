package io.bosca.workflow.media.image

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.serialization.Serializable
import java.awt.image.BufferedImage
import java.io.IOException
import javax.imageio.ImageIO


@Serializable
data class IfSquareConfiguration(
    val negate: Boolean = false,
    val expression: String,
    val workflows: List<String> = emptyList(),
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
        val cfg = getConfiguration<IfSquareConfiguration>(job)
        val response = withContext(Dispatchers.IO) {
            try {
                val imageFile = getContentFile(context, job)
                val image: BufferedImage = ImageIO.read(imageFile)
                val width = image.width
                val height = image.height
                width == height
            } catch (e: IOException) {
                System.err.println("Error reading image or checking dimensions: " + e.message)
                false
            }
        }
        if (response != cfg.negate) {
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