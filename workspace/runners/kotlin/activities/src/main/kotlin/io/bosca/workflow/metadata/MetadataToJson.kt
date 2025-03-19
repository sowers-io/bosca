package io.bosca.workflow.metadata

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.databind.SerializationFeature
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import io.bosca.api.Client
import io.bosca.graphql.fragment.Collection
import io.bosca.graphql.fragment.MetadataRelationship
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

class MetadataToJson(client: Client) : Activity(client) {

    override val id = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Convert Metadata Details to JSON",
            description = "Convert Metadata Details to JSON",
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
        val metadata = job.metadata?.metadata ?: error("No Metadata Found")
        val ow = ObjectMapper()
            .disable(SerializationFeature.WRITE_DATES_AS_TIMESTAMPS)
            .registerModules(JavaTimeModule())
            .writer()
            .withDefaultPrettyPrinter()
        val json = ow.writeValueAsString(metadata)
        val content =
            if (metadata.content.metadataContent.type == "bosca/v-document" || metadata.content.metadataContent.type == "bosca/v-guide") {
                client.metadata.getDocument(metadata.id, metadata.version)?.asText()
            } else if (metadata.content.metadataContent.type.startsWith("text/")) {
                client.metadata.getTextContents(metadata.id)
            } else {
                null
            } ?: ""
        val relationships = client.metadata.getRelationships(metadata.id)
        val relationshipMap = mutableMapOf<String, MutableList<MetadataRelationship>>()
        for (relationship in relationships) {
            val list = relationshipMap[relationship.relationship]
            if (list == null) {
                relationshipMap[relationship.relationship] = mutableListOf(relationship)
            } else {
                list.add(relationship)
            }
        }
        val parents = client.metadata.getParents(metadata.id)
        val parentCollections = mutableListOf<Collection>()
        for (parent in parents) {
            client.collections.get(parent.id)?.let { parentCollections.add(it) }
        }
        val objectMap = json.parseToJsonElement().jsonObject + mapOf(
            "_type" to JsonPrimitive("metadata"),
            "_relationships" to ow.writeValueAsString(relationshipMap).parseToJsonElement(),
            "_parents" to ow.writeValueAsString(parentCollections).parseToJsonElement(),
            "_content" to JsonPrimitive(content)
        )
        val documentObject = JsonObject(objectMap).toString()
        setSupplementaryContents(job, OUTPUT_NAME, "JSON", documentObject, "application/json")
    }

    companion object {
        const val ID = "metadata.to.json"
        const val OUTPUT_NAME = "supplementary"
    }
}