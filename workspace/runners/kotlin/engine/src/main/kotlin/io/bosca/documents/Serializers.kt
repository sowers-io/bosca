package io.bosca.documents

import io.bosca.documents.marks.*
import kotlinx.serialization.modules.SerializersModule
import kotlinx.serialization.modules.polymorphic
import kotlinx.serialization.modules.subclass

val DocumentSerializers = SerializersModule {
    polymorphic(DocumentNode::class) {
        subclass(Document::class, Document.serializer())
        subclass(HeadingNode::class, HeadingNode.serializer())
        subclass(ParagraphNode::class, ParagraphNode.serializer())
        subclass(TextNode::class, TextNode.serializer())
        subclass(HtmlNode::class, HtmlNode.serializer())
        subclass(BibleNode::class, BibleNode.serializer())
        subclass(SuperscriptNode::class, SuperscriptNode.serializer())
        subclass(BlockquoteNode::class, BlockquoteNode.serializer())
        subclass(HardBreakNode::class, HardBreakNode.serializer())
        subclass(BulletListNode::class, BulletListNode.serializer())
        subclass(OrderedListNode::class, OrderedListNode.serializer())
        subclass(ListItemNode::class, ListItemNode.serializer())
        subclass(HorizontalRuleNode::class, HorizontalRuleNode.serializer())
        subclass(ContainerNode::class, ContainerNode.serializer())
    }
    polymorphic(Mark::class) {
        subclass(Bold::class, Bold.serializer())
        subclass(Italic::class, Italic.serializer())
        subclass(Link::class, Link.serializer())
        subclass(Hidden::class, Hidden.serializer())
        subclass(Superscript::class, Superscript.serializer())
        subclass(Subscript::class, Subscript.serializer())
    }
}