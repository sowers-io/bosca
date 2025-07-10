package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.documents.BulletListNode
import io.bosca.documents.ContainerNode
import io.bosca.documents.Content
import io.bosca.documents.DocumentNode
import io.bosca.documents.HeadingNode
import io.bosca.documents.ListItemNode
import io.bosca.documents.OrderedListNode
import io.bosca.documents.ParagraphNode
import io.bosca.documents.TextNode
import io.bosca.graphql.fragment.Document
import io.bosca.util.decode

suspend fun Document.asText(client: Client, configuration: DocumentToTextConfiguration = DocumentToTextConfiguration()): String {
    val content = content.decode<Content>()
    val string = StringBuilder()
    content?.document?.let {
        if (!configuration.includeTitle) {
            var first = true
            val nodes = it.content.filter {
                if (it is HeadingNode && it.attributes.level == 1 && first) {
                    first = false
                    false
                } else {
                    true
                }
            }
            for (node in nodes) {
                append(node, client, string, configuration)
            }
        } else {
            append(it, client, string, configuration)
        }
        string.append("\n\n")
    }
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

private suspend fun append(node: DocumentNode, client: Client, builder: StringBuilder, configuration: DocumentToTextConfiguration) {
    if (node is ParagraphNode) {
        builder.append("\n")
    }
    if (node is OrderedListNode || node is BulletListNode || node is ListItemNode) {
        builder.append("\n")
    }
    if (node is TextNode) {
        builder.append(node.text.replace("’", "'"))
    }
    try {
        if (node is ContainerNode) {
            if (node.attributes.name != null && configuration.excludeContainers.contains(node.attributes.name)) return
            if (node.attributes.metadataId != null && node.attributes.references?.isNotEmpty() == true) {
                for (reference in node.attributes.references) {
                    val content = client.metadata.getBibleChapterContent(
                        node.attributes.metadataId ?: error("missing bible metadata id"), null, reference
                    )
                    if (content == null) continue
                    builder.append("\n------------------\n")
                    builder.append(content.reference.human)
                    builder.append("\n\n")
                    @Suppress("UNCHECKED_CAST")
                    append(content.component as Map<String, Any>, null, builder)
                    builder.append("\n------------------\n")
                }
            }
        }
    } catch (e: Exception) {
        println("error: failed to get bible chapter: ${e.message}")
    }
    for (child in node.content) {
        append(child, client, builder, configuration)
    }
    if (node is OrderedListNode || node is BulletListNode) {
        builder.append("\n")
    }
    if (node is ParagraphNode) {
        builder.append("\n\n")
    }
    if (node is HeadingNode) {
        builder.append("\n\n\n")
    }
}
