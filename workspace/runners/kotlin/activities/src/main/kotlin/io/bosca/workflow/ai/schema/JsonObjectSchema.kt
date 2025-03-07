package io.bosca.workflow.ai.schema

import dev.langchain4j.model.chat.request.json.JsonObjectSchema as Element
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
@SerialName("object")
data class JsonObjectSchema(
    val description: String? = null,
    val required: List<String> = emptyList(),
    val additionalProperties: Boolean = false,
    val definitions: Map<String, JsonSchemaElement> = emptyMap()
) : JsonSchemaElement {

    override fun toSchemaElement(): Element =
        Element.builder().description(description)
            .required(required)
            .additionalProperties(additionalProperties)
            .definitions(definitions.mapValues { it.value.toSchemaElement() })
            .build()
}