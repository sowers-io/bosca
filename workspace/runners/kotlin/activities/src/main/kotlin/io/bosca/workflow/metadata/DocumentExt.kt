package io.bosca.workflow.metadata

import io.bosca.api.Client
import io.bosca.documents.ContainerNode
import io.bosca.documents.Content
import io.bosca.documents.DocumentNode
import io.bosca.documents.TextNode
import io.bosca.graphql.fragment.Document
import io.bosca.util.decode
import io.bosca.util.json

suspend fun Document.asText(client: Client): String {
    val content = content.decode<Content>()
    val string = StringBuilder()
    content?.document?.let { append(it, client, string) }
    return string.toString().trim()
}

private suspend fun append(node: DocumentNode, client: Client, builder: StringBuilder) {
    if (node is TextNode) {
        builder.append(node.text)
        builder.append(" ")
    }
    try {
        if (node is ContainerNode && node.attributes.metadataId != null && node.attributes.references?.isNotEmpty() == true) {
            for (reference in node.attributes.references) {
                val content = client.metadata.getBibleChapterContent(node.attributes.metadataId!!, null, reference)
                if (content == null) continue
                builder.append("\r\n")
                builder.append(content.reference.human)
                builder.append("\r\n")

            }
        }
    } catch (e: Exception) {
        println("error: failed to get bible chapter: ${e.message}")
    }
    for (child in node.content) {
        append(child, client, builder)
    }
}
