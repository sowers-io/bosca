package io.bosca.source

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.workflow.ActivityContext
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.io.File

abstract class Handler {

    protected lateinit var client: Client
        private set
    protected lateinit var job: WorkflowJob
        private set
    protected lateinit var context: ActivityContext
        private set

    open fun initialize(client: Client, job: WorkflowJob, context: ActivityContext) {
        this.client = client
        this.job = job
        this.context = context
    }

    open suspend fun download(): File? {
        job.metadata?.metadata?.source?.sourceUrl?.let {
            val file = withContext(Dispatchers.IO) {
                val extension = (job.metadata?.metadata?.content?.type?.split("/")?.last() ?: ".download")
                File.createTempFile(job.id.id, ".$extension")
            }
            context.addFile(file)
            client.files.download(it, file)
            return file
        }
        return null
    }
}