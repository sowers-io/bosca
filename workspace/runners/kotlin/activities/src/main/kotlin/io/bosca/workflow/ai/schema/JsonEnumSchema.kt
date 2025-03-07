package io.bosca.workflow.ai.schema

import dev.langchain4j.model.chat.request.json.JsonEnumSchema as Element
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
@SerialName("enum")
data class JsonEnumSchema(
    val description: String? = null,
    val enumValues: List<String>? = null
) : JsonSchemaElement {

    override fun toSchemaElement(): Element = Element.builder().description(description).enumValues(enumValues).build()
}