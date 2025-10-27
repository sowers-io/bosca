package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.documents.Content
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.ActivityParameterInput
import io.bosca.graphql.type.ActivityParameterType
import io.bosca.util.decode
import io.bosca.util.toJsonElement
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

class DocumentToJson(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Document to Json",
            description = "Document to Json",
            inputs = emptyList(),
            outputs = listOf(
                ActivityParameterInput(name = OUTPUT_NAME, type = ActivityParameterType.SUPPLEMENTARY)
            ),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val document = client.metadata.getDocument(
            job.metadata?.metadata?.id ?: error("metadata id is missing"),
            job.metadata?.metadata?.version ?: error("metadata version is missing")
        )
        if (document == null) {
            setSupplementaryContents(job, OUTPUT_NAME, "Document JSON", "{}", "text/json")
            return
        }
        val content = document.content.decode<Content>()
        setSupplementaryContents(job, OUTPUT_NAME, "Document JSON", Json.encodeToString(content), "text/json")
    }

    companion object {
        const val ID = "metadata.document.to.json"
        const val OUTPUT_NAME = "json"
    }
}
