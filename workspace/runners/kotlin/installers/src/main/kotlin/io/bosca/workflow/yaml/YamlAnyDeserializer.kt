package io.bosca.workflow.yaml

import com.charleskorn.kaml.*
import kotlinx.serialization.InternalSerializationApi
import kotlinx.serialization.KSerializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.SerialKind
import kotlinx.serialization.descriptors.buildSerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder

object YamlAnyDeserializer : KSerializer<Any> {
    @OptIn(InternalSerializationApi::class)
    override val descriptor: SerialDescriptor = buildSerialDescriptor("Any", SerialKind.CONTEXTUAL)

    override fun deserialize(decoder: Decoder): Any {
        val yaml = decoder as? YamlInput ?: error("Can be deserialized only by YAML")
        val path = yaml.getCurrentPath()
        val key = path.segments.dropLast(1).last() as YamlPathSegment.MapElementKey
        val node = yaml.node.yamlMap.get<YamlNode>(key.key) ?: error("Missing value for key $key")
        return deserializeNode(node) ?: emptyMap<String, Any>()
    }

    private fun deserializeNode(node: YamlNode): Any? = when (node) {
        is YamlScalar -> {
            val content = node.content
            when {
                content == "null" -> emptyMap<String, Any>()
                content == "true" || content == "false" -> content.toBoolean()
                content.toIntOrNull() != null -> content.toInt()
                content.toDoubleOrNull() != null -> content.toDouble()
                else -> content
            }
        }
        is YamlMap -> {
            val map = mutableMapOf<String, Any>()
            node.entries.forEach { (key, value) ->
                deserializeNode(value)?.let { map[key.content] = it }
            }
            map
        }
        is YamlList -> node.items.map { deserializeNode(it) }
        is YamlNull -> null
        else -> error("Unsupported YAML node type")
    }

    override fun serialize(encoder: Encoder, value: Any) {
        error("Serialization is not supported")
    }
}
