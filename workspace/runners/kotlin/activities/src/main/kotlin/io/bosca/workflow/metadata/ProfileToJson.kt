package io.bosca.workflow.metadata

import com.fasterxml.jackson.databind.ObjectMapper
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

class ProfileToJson(client: Client) : Activity(client) {

    override val id = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Convert Profile Details to JSON",
            description = "Convert Profile Details to JSON",
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
        val profile = job.profile?.profile ?: error("No Profile Found")
        if (profile.collection == null) {
            client.profiles.addCollection(profile.id)
            throw Exception("Collection not found for profile ${profile.id}, retry after collection is added")
        }
        val ow = ObjectMapper().registerModules(JavaTimeModule()).writer().withDefaultPrettyPrinter()
        val json = ow.writeValueAsString(profile)
        val documentObject = JsonObject(
            (json.parseToJsonElement().jsonObject + mapOf(
                "_type" to JsonPrimitive("profile"),
                "_content" to JsonPrimitive("")
            ))
        ).toString()
        try {
            val contents = getInputSupplementaryText(context, job, OUTPUT_NAME)
            if (contents != json) {
                setSupplementaryContents(job, OUTPUT_NAME, "JSON", documentObject, "application/json")
            }
        } catch (e: Exception) {
            setSupplementaryContents(job, OUTPUT_NAME, "JSON", documentObject, "application/json")
        }
    }

    companion object {
        const val ID = "profile.to.json"
        const val OUTPUT_NAME = "supplementary"
    }
}