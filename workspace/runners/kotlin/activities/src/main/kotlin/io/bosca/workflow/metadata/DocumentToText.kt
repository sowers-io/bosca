package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.ActivityParameterInput
import io.bosca.graphql.type.ActivityParameterType
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.serialization.Serializable

@Serializable
data class DocumentToTextConfiguration(
    val includeTitle: Boolean = true,
    val excludeContainers: Set<String> = emptySet()
)

class DocumentToText(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Document to Text",
            description = "Document to Text",
            inputs = emptyList(),
            outputs = listOf(
                ActivityParameterInput(name = OUTPUT_NAME, type = ActivityParameterType.SUPPLEMENTARY)
            ),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val configuration = getConfiguration<DocumentToTextConfiguration>(job)
        val document = client.metadata.getDocument(
            job.metadata?.metadata?.id ?: error("metadata id is missing"),
            job.metadata?.metadata?.version ?: error("metadata version is missing")
        ) ?: error("missing document")
        val content = document.asText(client, configuration)
        setSupplementaryContents(job, OUTPUT_NAME, "Document Text", content.trim(), "text/plain")
    }

    companion object {
        const val ID = "metadata.document.to.text"
        const val OUTPUT_NAME = "text"
    }
}
