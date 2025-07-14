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
import kotlin.concurrent.atomics.AtomicLong
import kotlin.concurrent.atomics.ExperimentalAtomicApi

fun DocumentNode.filterTitle(title: Boolean): List<DocumentNode> {
    var first = true
    return content.filter {
        if (it is HeadingNode && it.attributes.level == 1 && first) {
            first = false
            title
        } else {
            !title
        }
    }
}

@OptIn(ExperimentalAtomicApi::class)
suspend fun Document.asText(client: Client, configuration: DocumentToTextConfiguration = DocumentToTextConfiguration()): String {
    val content = content.decode<Content>()
    val string = StringBuilder()
    val length = AtomicLong(0)
    content?.document?.let {
        if (!configuration.includeTitle) {
            val nodes = it.filterTitle(false)
            for (node in nodes) {
                append(node, client, string, configuration, length)
            }
        } else if(configuration.includeTtsMarkup) {
            val title = it.filterTitle(true).firstOrNull()
            title?.let {
                append(it, client, string, configuration, length)
            }
            val nodes = it.filterTitle(false)
            for (node in nodes) {
                append(node, client, string, configuration, length)
            }
        } else {
            append(it, client, string, configuration, length)
        }
        string.append("\n")
    }
    return string.toString().trim()
}

@OptIn(ExperimentalAtomicApi::class)
private fun append(component: Any, key: String?, builder: StringBuilder, configuration: DocumentToTextConfiguration, length: AtomicLong) {
    if (component is Map<*, *>) {
        component.entries.forEach { entry ->
            append(entry.value!!, entry.key as String, builder, configuration, length)
        }
    } else if (component is List<*>) {
        component.forEach { append(it!!, key, builder, configuration, length) }
    } else if (component is String && key == "text") {
        builder.append(component, configuration, length)
        builder.append(" ", configuration, length)
    }
}

@OptIn(ExperimentalAtomicApi::class)
private fun StringBuilder.append(text: String, configuration: DocumentToTextConfiguration, length: AtomicLong) {
    if (configuration.includeTtsMarkup) {
        val len = length.addAndFetch(text.toByteArray().size.toLong())
        if (len > 4000) {
            length.store(0)
            append("[split]")
        }
    }
    append(text)
}

@OptIn(ExperimentalAtomicApi::class)
private suspend fun append(node: DocumentNode, client: Client, builder: StringBuilder, configuration: DocumentToTextConfiguration, length: AtomicLong) {
    if (node is ParagraphNode) {
        builder.append("\n", configuration, length)
    }
    if (node is OrderedListNode || node is BulletListNode || node is ListItemNode) {
        builder.append("\n", configuration, length)
    }
    if (node is TextNode) {
        builder.append(node.text.replace("’", "'"), configuration, length)
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
                    builder.append(content.reference.human, configuration, length)
                    if (configuration.includeTtsMarkup) {
                        builder.append(" [pause] ")
                    }
                    builder.append("\n\n", configuration, length)
                    @Suppress("UNCHECKED_CAST")
                    append(content.component as Map<String, Any>, null, builder, configuration, length)
                    if (configuration.includeTtsMarkup) {
                        builder.append(" [pause] ", configuration, length)
                    }
                }
            }
        }
    } catch (e: Exception) {
        println("error: failed to get bible chapter: ${e.message}")
    }
    for (child in node.content) {
        append(child, client, builder, configuration, length)
    }
    if (node is OrderedListNode || node is BulletListNode) {
        builder.append("\n", configuration, length)
    }
    if (node is ParagraphNode) {
        if (configuration.includeTtsMarkup) {
            builder.append(" [pause] ", configuration, length)
        }
        builder.append("\n\n", configuration, length)
    }
    if (node is HeadingNode) {
        if (configuration.includeTtsMarkup) {
            builder.append(" [pause] ", configuration, length)
        }
        builder.append("\n\n\n", configuration, length)
    }
}
