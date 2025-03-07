package io.bosca.workflow.yaml

import com.charleskorn.kaml.*
import io.bosca.workflow.ai.schema.JsonSchemaSerializers
import io.bosca.workflow.ext.ModelConfigurationSerializers
import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.modules.plus
import java.io.File

object YamlLoader {

    private val yaml = Yaml(JsonSchemaSerializers + ModelConfigurationSerializers)

    fun <T> load(serializer: DeserializationStrategy<T>, directory: File, file: File): T {
        val contents = file.readText()
        val node = yaml.parseToYamlNode(contents)
        val processed = processReferences(node, directory)
        return yaml.decodeFromYamlNode(serializer, processed)
    }

    private fun processReferences(node: YamlNode, directory: File): YamlNode {
        return when (node) {
            is YamlMap -> {
                val refValue = node.get<YamlNode>("\$ref")
                if (refValue is YamlScalar) {
                    val path = refValue.content
                    val contents = File(directory, path).readText()
                    val newNode = yaml.parseToYamlNode(contents)
                    processReferences(newNode, directory)
                } else {
                    val processedEntries = node.entries.entries.associate { (key, value) ->
                        key to processReferences(value, directory)
                    }
                    YamlMap(processedEntries, node.path)
                }
            }

            is YamlList -> {
                YamlList(node.items.map { processReferences(it, directory) }, node.path)
            }

            else -> node
        }
    }
}
