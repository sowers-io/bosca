package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
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
        job.metadata?.metadata?.source?.sourceUrl?.let {
            val file = downloadToFile(context, job, it)
            setContent(context, job, file)
        }
    }

    companion object {
        const val ID = "metadata.download.source.url"
    }
}