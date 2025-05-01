package io.bosca.workflow.ai.schema

import dev.langchain4j.model.chat.request.json.JsonReferenceSchema as Element
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
@SerialName("reference")
data class JsonReferenceSchema(
    val reference: String
) : JsonSchemaElement {

    override fun toSchemaElement(): Element = Element.builder().reference(reference).build()
}