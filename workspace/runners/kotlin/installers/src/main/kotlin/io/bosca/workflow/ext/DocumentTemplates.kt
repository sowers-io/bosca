package io.bosca.workflow.ext

import io.bosca.documents.*
import io.bosca.documents.Content
import io.bosca.documents.Document
import io.bosca.graphql.type.*
import io.bosca.util.encode
import io.bosca.util.toOptional
import io.bosca.workflow.models.DocumentTemplateDefinition
import io.bosca.workflow.models.ContentNode

private fun toDocumentNode(node: ContentNode): DocumentNode {
    return when (node.type) {
        "text" -> TextNode(text = node.text ?: "")
        "image" -> ImageNode(
            attributes = ImageAttributes(
                src = node.attributes?.get("src") ?: "/placeholder.svg"
            )
        )

        "horizontalRule" -> HorizontalRuleNode()
        "hardBreak" -> HardBreakNode()
        "html" -> HtmlNode(
            html = node.text ?: ""
        )

        else -> {
            val content = node.content?.map { toDocumentNode(it) } ?: emptyList()
            when (node.type) {
                "heading" -> HeadingNode(
                    attributes = HeadingAttributes(level = node.attributes?.get("level")?.toInt() ?: 1),
                    content = content
                )

                "container" -> ContainerNode(
                    attributes = ContainerAttributes(
                        classes = node.attributes?.get("class"),
                        name = node.attributes?.get("name")
                    ),
                    content = content
                )

                "paragraph" -> ParagraphNode(content = content)
                "bulletList" -> BulletListNode(
                    attributes = BulletListAttributes(classes = node.attributes?.get("class")),
                    content = content
                )

                "orderedList" -> OrderedListNode(
                    attributes = OrderedListAttributes(classes = node.attributes?.get("class")),
                    content = content
                )

                "listItem" -> ListItemNode(
                    attributes = ListItemAttributes(classes = node.attributes?.get("class")),
                    content = content
                )

                "blockquote" -> BlockquoteNode(content = content)
                "superscript" -> SuperscriptNode(content = content)
                "bible" -> BibleNode(
                    attributes = BibleAttributes(
                        classes = node.attributes?.get("class"),
                        references = listOfNotNull(
                            BibleReference(usfm = node.attributes?.get("usfm"), human = node.attributes?.get("human"))
                        )
                    ),
                    content = content
                )

                else -> ParagraphNode()
            }
        }
    }
}

fun DocumentTemplateDefinition.toInput() = DocumentTemplateInput(
    configuration = document.configuration.toOptional(),
    containers = document.containers.map {
        DocumentTemplateContainerInput(
            id = it.id,
            name = it.name,
            description = it.description,
            supplementaryKey = it.supplementary.toOptional(),
            workflows = it.workflows.map { workflow ->
                TemplateWorkflowInput(
                    autoRun = workflow.autoRun,
                    workflowId = workflow.workflow
                )
            },
        )
    }.toOptional(),
    defaultAttributes = document.defaultAttributes.toOptional(),
    content = Content(Document(content = document.content.document.content.map { toDocumentNode(it) })).encode(),
    attributes = document.attributes.map { it.toInput() }
)
