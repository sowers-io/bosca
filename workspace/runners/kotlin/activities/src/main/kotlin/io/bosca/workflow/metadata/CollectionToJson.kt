package io.bosca.workflow.metadata

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.databind.SerializationFeature
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.ActivityParameterInput
import io.bosca.graphql.type.ActivityParameterType
import io.bosca.util.parseToJsonElement
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.jsonObject

class CollectionToJson(client: Client) : Activity(client) {

    override val id = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Convert Collection Details to JSON",
            description = "Convert Collection Details to JSON",
            inputs = emptyList(),
            outputs = listOf(
                ActivityParameterInput(
                    OUTPUT_NAME,
                    ActivityParameterType.SUPPLEMENTARY,
                )
            ),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val collection = job.collection?.collection ?: error("No Collection Found")
        val ow = ObjectMapper()
            .disable(SerializationFeature.WRITE_DATES_AS_TIMESTAMPS)
            .registerModules(JavaTimeModule())
            .writer()
            .withDefaultPrettyPrinter()
        val json = ow.writeValueAsString(collection)
        val documentObject = JsonObject(
            (json.parseToJsonElement().jsonObject + mapOf(
                "_type" to JsonPrimitive("collection"),
                "_content" to JsonPrimitive("")
            ))
        ).toString()
        setSupplementaryContents(job, OUTPUT_NAME, "JSON", documentObject, "application/json")
    }

    companion object {
        const val ID = "collection.to.json"
        const val OUTPUT_NAME = "supplementary"
    }
}