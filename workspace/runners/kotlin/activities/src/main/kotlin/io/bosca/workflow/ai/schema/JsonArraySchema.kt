package io.bosca.workflow.ai.schema

import dev.langchain4j.model.chat.request.json.JsonArraySchema as Element
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
@SerialName("array")
data class JsonArraySchema(
    val description: String? = null,
    val items: JsonSchemaElement? = null
) : JsonSchemaElement {

    override fun toSchemaElement(): Element =
        Element.builder().description(description).items(items?.toSchemaElement()).build()
}