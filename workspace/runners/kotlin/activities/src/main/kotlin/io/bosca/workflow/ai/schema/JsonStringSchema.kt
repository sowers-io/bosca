package io.bosca.workflow.ai.schema

import dev.langchain4j.model.chat.request.json.JsonStringSchema as Element
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
@SerialName("string")
data class JsonStringSchema(
    val description: String? = null
) : JsonSchemaElement {

    override fun toSchemaElement(): Element = Element.builder().description(description).build()
}