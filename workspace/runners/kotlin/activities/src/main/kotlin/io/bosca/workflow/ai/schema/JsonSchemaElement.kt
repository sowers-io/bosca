package io.bosca.workflow.ai.schema

import dev.langchain4j.model.chat.request.json.JsonSchemaElement as Element
import kotlinx.serialization.Serializable

@Serializable
sealed interface JsonSchemaElement {

    fun toSchemaElement(): Element
}
