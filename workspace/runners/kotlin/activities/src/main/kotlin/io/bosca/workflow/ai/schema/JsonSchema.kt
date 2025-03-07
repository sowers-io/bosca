package io.bosca.workflow.ai.schema

import kotlinx.serialization.Serializable
import kotlinx.serialization.modules.SerializersModule
import kotlinx.serialization.modules.polymorphic

@Serializable
data class JsonSchema(
    val name: String,
    val rootElement: JsonSchemaElement
)

val JsonSchemaSerializers = SerializersModule {
    polymorphic(JsonSchemaElement::class) {
        subclass(JsonArraySchema::class, JsonArraySchema.serializer())
        subclass(JsonObjectSchema::class, JsonObjectSchema.serializer())
        subclass(JsonStringSchema::class, JsonStringSchema.serializer())
        subclass(JsonEnumSchema::class, JsonEnumSchema.serializer())
        subclass(JsonBooleanSchema::class, JsonBooleanSchema.serializer())
        subclass(JsonIntegerSchema::class, JsonIntegerSchema.serializer())
        subclass(JsonNumberSchema::class, JsonNumberSchema.serializer())
        subclass(JsonReferenceSchema::class, JsonReferenceSchema.serializer())
    }
}