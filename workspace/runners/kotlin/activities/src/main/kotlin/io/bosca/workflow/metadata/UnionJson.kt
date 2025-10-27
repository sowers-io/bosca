package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.ActivityParameterInput
import io.bosca.graphql.type.ActivityParameterType
import io.bosca.util.parseToJsonElement
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject

class UnionJson(client: Client) : Activity(client) {

    override val id = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Union JSON",
            description = "Union JSON",
            inputs = listOf(
                ActivityParameterInput(
                    JSON1,
                    ActivityParameterType.SUPPLEMENTARY,
                ),
                ActivityParameterInput(
                    JSON2,
                    ActivityParameterType.SUPPLEMENTARY,
                )
            ),
            outputs = listOf(
                ActivityParameterInput(
                    OUTPUT_NAME,
                    ActivityParameterType.SUPPLEMENTARY,
                )
            ),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val json1Str = getInputSupplementaryText(context, job, JSON1) ?: error("No JSON1 Found")
        val json2Str = getInputSupplementaryText(context, job, JSON2) ?: error("No JSON2 Found")

        val json1 = json1Str.parseToJsonElement()
        val json2 = json2Str.parseToJsonElement()

        val json3 = if (json1 is JsonObject && json2 is JsonObject) {
            JsonObject(json1.jsonObject + json2.jsonObject).toString()
        } else if (json1 is JsonArray && json2 is JsonArray) {
            JsonArray(json1.jsonArray + json2.jsonArray).toString()
        }  else {
            error("JSON1 and JSON2 are not of the same type")
        }

        setSupplementaryContents(job, OUTPUT_NAME, "JSON", json3, "application/json")
    }

    companion object {
        const val ID = "metadata.union.json"

        const val JSON1 = "json1"
        const val JSON2 = "json2"

        const val OUTPUT_NAME = "supplementary"
    }
}