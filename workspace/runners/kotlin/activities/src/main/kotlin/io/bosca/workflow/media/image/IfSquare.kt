package io.bosca.workflow.media.image

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.withContext
import kotlinx.serialization.Serializable
import javax.imageio.ImageIO
import kotlin.concurrent.atomics.AtomicInt
import kotlin.concurrent.atomics.ExperimentalAtomicApi
import kotlin.concurrent.atomics.decrementAndFetch
import kotlin.concurrent.atomics.incrementAndFetch

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

    @OptIn(ExperimentalAtomicApi::class)
    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val ctx = getContext<IfSquareContext>(job)
        if (ctx.executed) return
        if (!(job.metadata?.metadata?.content?.metadataContent?.type ?: "").startsWith("image/")) return
        val cfg = getConfiguration<IfSquareConfiguration>(job)
        val file = getContentFile(context, job)
        while (running.load() > 5) {
            delay(10)
        }
        running.incrementAndFetch()
        try {
            val square = withContext(Dispatchers.IO) {
                try {
                    val metadata = ImageIO.read(file)
                    if (metadata == null) return@withContext false
                    metadata.width == metadata.height
                } catch (_: IllegalArgumentException) {
                    false
                }
            }
            if (square != cfg.negate) {
                client.workflows.enqueueChildWorkflows(
                    cfg.workflows,
                    job.id
                )
                setContext(job, IfSquareContext(executed = true))
            }
        } finally {
            running.decrementAndFetch()
        }
    }

    companion object {

        @OptIn(ExperimentalAtomicApi::class)
        private val running = AtomicInt(0)

        const val ID = "workflow.image.if.square"
    }
}