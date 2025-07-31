package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.util.toJsonElement
import io.bosca.util.toOptional
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.serialization.json.contentOrNull
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import kotlinx.serialization.json.longOrNull
import java.text.DateFormat
import java.time.ZoneOffset
import java.time.ZonedDateTime
import java.util.Date

class BeginTransitionTo(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Begin Metadata Transition",
            description = "Begin a Metadata Transition",
            configuration = mapOf<String, Any>(
                "state" to "draft",
            ).toOptional(),
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val configuration = job.workflowActivity.workflowActivity.configuration as Map<*, *>
        val requiredState = configuration["requiredState"] as? String
        if (requiredState != null && requiredState != job.metadata?.metadata?.workflow?.metadataWorkflow?.state) {
            return
        }
        val state = configuration["state"] as String
        val current = job.metadata?.metadata?.workflow?.metadataWorkflow
        if (state != current?.state && state != current?.pending) {
            val states = client.workflows.getStates()
            val published = states.first { it.id == "published" }
            var stateValid: ZonedDateTime? = null
            if (state == published.id) {
                var date: Date? = null
                job.metadata?.metadata?.let { metadata ->
                    metadata.attributes?.toJsonElement()?.let { attributes ->
                        val published = attributes.jsonObject.getValue("published").jsonPrimitive.longOrNull
                        if (published != null) {
                            date = Date(published)
                        } else {
                            val published = attributes.jsonObject.getValue("published").jsonPrimitive.contentOrNull
                            if (published != null) {
                                date = DateFormat.getDateTimeInstance().parse(published)
                            }
                        }
                    }
                }
                if (date != null && date.after(Date(System.currentTimeMillis()))) {
                    stateValid = ZonedDateTime.ofInstant(date.toInstant(), ZoneOffset.UTC)
                }
            }
            client.workflows.beginMetadataTransition(
                id = job.metadata?.metadata?.id ?: error("missing metadata"),
                version = job.metadata!!.metadata.version,
                state = state,
                status = "Begin Metadata Transition",
                stateValid = stateValid,
            )
        }
    }

    companion object {
        const val ID = "metadata.begin.transition.to"
    }
}