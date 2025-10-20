package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.source.DefaultHandler
import io.bosca.source.Handler
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext

class DownloadSourceUrl(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Download Source URL",
            description = "Download Source URL and Upload as Metadata Content (if not already uploaded)",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        if (job.metadata?.metadata?.uploaded != null) return
        val handler = job.metadata?.metadata?.source?.source?.configuration?.let {
            val cfg = it as Map<*, *>
            cfg["handler"]?.let {
                val clazz = Class.forName(it.toString())
                if (Handler::class.java.isAssignableFrom(clazz)) {
                    clazz.getDeclaredConstructor().newInstance() as Handler
                } else {
                    error("Unsupported handler type: ${clazz.simpleName}")
                }
            }
        } ?: DefaultHandler()
        handler.initialize(client, job, context)
        handler.download()?.let {
            setContent(job, it)
        }
    }

    companion object {
        const val ID = "metadata.download.source.url"
    }
}