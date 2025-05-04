package io.bosca.documents

import io.bosca.documents.marks.Mark
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class ContainerAttributes(
    @SerialName("class")
    override val classes: String? = null,
    val name: String? = null,
    val metadataId: String? = null,
    val references: List<String>? = null
) : DocumentAttributes {

    override fun withClasses(classes: String?): ContainerAttributes {
        return copy(classes = classes)
    }

    fun withReferences(metadataId: String?, references: List<String>?): ContainerAttributes {
        if (references == null || references.isEmpty()) return copy(metadataId = null, references = null)
        return copy(metadataId = metadataId, references = references)
    }
}

@Serializable
@SerialName("container")
data class ContainerNode(
    @SerialName("attrs")
    override val attributes: ContainerAttributes,
    override val content: List<DocumentNode> = emptyList(),
    override val marks: List<Mark> = emptyList(),
) : DocumentNode