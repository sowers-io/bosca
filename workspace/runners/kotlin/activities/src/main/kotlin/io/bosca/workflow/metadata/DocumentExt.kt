package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.documents.ContainerNode
import io.bosca.documents.Content
import io.bosca.documents.DocumentNode
import io.bosca.documents.TextNode
import io.bosca.graphql.fragment.Document
import io.bosca.util.decode

suspend fun Document.asText(client: Client, excludeContainers: Set<String> = emptySet()): String {
    val content = content.decode<Content>()
    val string = StringBuilder()
    content?.document?.let { append(it, client, string, excludeContainers) }
    return string.toString().trim()
}

private fun append(component: Any, key: String?, builder: StringBuilder) {
    if (component is Map<*, *>) {
        component.entries.forEach { entry ->
            append(entry.value!!, entry.key as String, builder)
        }
    } else if (component is List<*>) {
        component.forEach { append(it!!, key, builder) }
    } else if (component is String && key == "text") {
        builder.append(component)
        builder.append(" ")
    }
}

private suspend fun append(node: DocumentNode, client: Client, builder: StringBuilder, excludeContainers: Set<String>) {
    if (node is TextNode) {
        builder.append(node.text)
        builder.append(" ")
    }
    try {
        if (node is ContainerNode) {
            if (node.attributes.name != null && excludeContainers.contains(node.attributes.name)) return
            if (node.attributes.metadataId != null && node.attributes.references?.isNotEmpty() == true) {
                for (reference in node.attributes.references) {
                    val content = client.metadata.getBibleChapterContent(
                        node.attributes.metadataId ?: error("missing bible metadata id"), null, reference
                    )
                    if (content == null) continue
                    builder.append("\r\n------------------\r\n")
                    builder.append(content.reference.human)
                    builder.append("\r\n\r\n")
                    @Suppress("UNCHECKED_CAST")
                    append(content.component as Map<String, Any>, null, builder)
                    builder.append("\r\n------------------\r\n")
                }
            }
        }
    } catch (e: Exception) {
        println("error: failed to get bible chapter: ${e.message}")
    }
    for (child in node.content) {
        append(child, client, builder, excludeContainers)
    }
}
