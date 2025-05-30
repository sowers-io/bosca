package io.bosca.documents

import io.bosca.documents.marks.Mark
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
@SerialName("horizontalRule")
data class HorizontalRuleNode(
    @SerialName("attrs")
    override val attributes: DocumentAttributes = EmptyDocumentAttributes(),
    override val content: List<DocumentNode> = emptyList(),
    override val marks: List<Mark> = emptyList(),
) : DocumentNode