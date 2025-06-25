package io.bosca.documents

import io.bosca.documents.marks.Mark
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class TableRowNodeAttributes(
    @SerialName("class")
    override val classes: String? = null,
) : DocumentAttributes {

    override fun withClasses(classes: String?): TableRowNodeAttributes {
        return copy(classes = classes)
    }
}

@Serializable
@SerialName("tableRow")
data class TableRowNode(
    @SerialName("attrs")
    override val attributes: TableRowNodeAttributes = TableRowNodeAttributes(),
    override val content: List<DocumentNode> = emptyList(),
    override val marks: List<Mark> = emptyList(),
) : DocumentNode
