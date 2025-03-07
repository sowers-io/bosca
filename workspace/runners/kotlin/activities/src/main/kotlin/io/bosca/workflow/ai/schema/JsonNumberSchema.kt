package io.bosca.workflow.ai.schema

import dev.langchain4j.model.chat.request.json.JsonNumberSchema as Element
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
@SerialName("number")
data class JsonNumberSchema(
    val description: String? = null
) : JsonSchemaElement {

    override fun toSchemaElement(): Element = Element.builder().description(description).build()
}