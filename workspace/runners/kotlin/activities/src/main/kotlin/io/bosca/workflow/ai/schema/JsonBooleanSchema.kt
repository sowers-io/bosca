package io.bosca.workflow.ai.schema

import dev.langchain4j.model.chat.request.json.JsonBooleanSchema as Element
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
@SerialName("boolean")
data class JsonBooleanSchema(
    val description: String? = null
) : JsonSchemaElement {

    override fun toSchemaElement(): Element = Element.builder().description(description).build()
}