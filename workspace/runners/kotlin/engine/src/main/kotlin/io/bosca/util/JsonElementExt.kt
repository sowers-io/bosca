package io.bosca.util

import kotlinx.serialization.json.*

fun JsonElement.toAny(): Any? {
    return when (this) {
        is JsonObject -> mapValues { it.value.toAny() }
        is JsonArray -> toAnyList()
        is JsonPrimitive -> when {
            isString -> content
            booleanOrNull != null -> boolean
            intOrNull != null -> int
            floatOrNull != null -> float
            longOrNull != null -> long
            doubleOrNull != null -> double
            else -> null
        }

        JsonNull -> null
        else -> error("Unsupported JSON element type: $this")
    }
}

fun JsonArray.toAnyList(): List<Any?> {
    val items = mutableListOf<Any?>()
    forEach { items.add(it.toAny()) }
    return items
}

fun Any?.toJsonElement(): JsonElement {
    if (this == null) return JsonNull
    @Suppress("UNCHECKED_CAST")
    return when (this) {
        is Map<*, *> -> JsonObject(mapValues { it.value?.toJsonElement() ?: JsonNull } as Map<String, JsonElement>)
        is List<*> -> JsonArray(map { it?.toJsonElement() ?: JsonNull })
        is Number -> JsonPrimitive(this)
        is String -> JsonPrimitive(this)
        is Boolean -> JsonPrimitive(this)
        else -> error("unsupported type: $this")
    }
}

var json: Json = Json.Default

inline fun <reified T> T.encode(): Any = json.encodeToJsonElement(this).toAny()!!

inline fun <reified T> Any?.decode(jsonOverride: Json? = null): T? {
    val element = toJsonElement()
    if (element !is JsonObject) return null
    return (jsonOverride ?: json).decodeFromJsonElement<T>(element)
}

fun String.parseToJsonElement(): JsonElement = json.parseToJsonElement(this)