package io.bosca.documents

import io.bosca.documents.marks.Mark
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class ParagraphAttributes(
    @SerialName("class")
    override val classes: String? = null,
    val textAlign: String? = null
) : DocumentAttributes {

    override fun withClasses(classes: String?): ParagraphAttributes {
        return copy(classes = classes)
    }
}

@Serializable
@SerialName("paragraph")
data class ParagraphNode(
    @SerialName("attrs")
    override val attributes: ParagraphAttributes = ParagraphAttributes(),
    override val content: List<DocumentNode> = emptyList(),
    override val marks: List<Mark> = emptyList(),
) : DocumentNode