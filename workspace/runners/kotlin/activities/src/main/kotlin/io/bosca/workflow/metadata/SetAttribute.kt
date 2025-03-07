package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.util.parseToJsonElement
import io.bosca.util.toAny
import io.bosca.util.toOptional
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
enum class AttributeType {
    @SerialName("string")
    STRING,

    @SerialName("json")
    JSON,

    @SerialName("int")
    INT,

    @SerialName("float")
    FLOAT,

    @SerialName("boolean")
    BOOLEAN
}

@Serializable
data class SetAttributeConfiguration(val attribute: String? = null, val type: AttributeType)

class SetAttribute(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Set Metadata Attribute",
            description = "Set Metadata Attribute",
            configuration = mapOf<String, Any>(
                "attribute" to "name",
                "type" to "string"
            ).toOptional(),
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val text = getInputSupplementaryText(context, job, INPUT_NAME)
        val configuration = getConfiguration<SetAttributeConfiguration>(job)

        val value = when (configuration.type) {
            AttributeType.STRING -> text
            AttributeType.JSON -> text.trim().parseToJsonElement().toAny()
            AttributeType.INT -> text.trim().takeIf { it.isNotEmpty() && it != "null" }?.toLong() ?: 0
            AttributeType.FLOAT -> text.trim().takeIf { it.isNotEmpty() && it != "null" }?.toDouble() ?: 0
            AttributeType.BOOLEAN -> text.trim().takeIf { it.isNotEmpty() && it != "null" }?.toBoolean() ?: false
        }

        if (configuration.attribute != null) {
            when (configuration.type) {
                AttributeType.STRING -> setAttribute<String>(job, configuration.attribute, value.toString())
                AttributeType.JSON -> setAttribute(job, configuration.attribute, text.trim().parseToJsonElement())
                AttributeType.INT -> setAttribute<Long>(
                    job,
                    configuration.attribute,
                    text.trim().takeIf { it.isNotEmpty() && it != "null" }?.toLong() ?: 0
                )

                AttributeType.FLOAT -> setAttribute<Double>(
                    job,
                    configuration.attribute,
                    text.trim().takeIf { it.isNotEmpty() && it != "null" }?.toDouble() ?: 0.0
                )
                AttributeType.BOOLEAN -> setAttribute<Boolean>(
                    job,
                    configuration.attribute,
                    text.trim().takeIf { it.isNotEmpty() && it != "null" }?.toBoolean() ?: false
                )
            }
        } else {
            setAttribute(job, value)
        }
    }

    companion object {
        const val ID = "metadata.set.attribute"
        const val INPUT_NAME = "value"
    }
}