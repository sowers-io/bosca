package io.bosca.workflow.metadata

import io.bosca.documents.Content
import io.bosca.documents.DocumentNode
import io.bosca.documents.TextNode
import io.bosca.graphql.fragment.Document
import io.bosca.util.decode

fun Document.asText(): String {
    val content = content.decode<Content>()
    val string = StringBuilder()
    content?.document?.let { append(it, string) }
    return string.toString().trim()
}

private fun append(node: DocumentNode, builder: StringBuilder) {
    if (node is TextNode) {
        builder.append(node.text)
        builder.append(" ")
    }
    for (child in node.content) {
        append(child, builder)
    }
}
