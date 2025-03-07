package io.bosca.documents

import io.bosca.documents.marks.Mark
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
@SerialName("html")
data class HtmlNode(
    override val marks: List<Mark> = emptyList(),
    @SerialName("attrs")
    override val attributes: DocumentAttributes = EmptyDocumentAttributes(),
    override val content: List<DocumentNode> = emptyList(),
    val html: String,
) : DocumentNode