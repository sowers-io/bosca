import {useEditor} from "@tiptap/vue-3";
import StarterKit from "@tiptap/starter-kit";
import {Image} from "~/lib/editor/image.ts";
import Underline from "@tiptap/extension-underline";
import TextAlign from "@tiptap/extension-text-align";
import Placeholder from "@tiptap/extension-placeholder";
import Commands from "~/lib/editor/commands.ts";
import Suggestion from "~/lib/editor/suggestion.ts";
import {toast} from "~/components/ui/toast";
import type {
    Document as BoscaDocument,
    DocumentTemplateFragment,
    MetadataFragment
} from "~/lib/graphql/graphql.ts";
import {Plugin} from "@tiptap/pm/state";
import Document from "@tiptap/extension-document";
import {type EditorEvents} from "@tiptap/core";

export function newEditor(
    document: BoscaDocument,
    template: DocumentTemplateFragment,
    metadata: MetadataFragment,
    uploader: Uploader,
    onUpdate: (props: EditorEvents['update']) => void,
) {
    const CustomDocument = Document.extend({
        content: template.configuration.content || 'heading block+',
        addProseMirrorPlugins() {
            return [
                new Plugin({
                    appendTransaction: (transactions, oldState, newState) => {
                        const {doc, tr} = newState
                        let h1Count = 0
                        doc.descendants((node, pos) => {
                            if (node.type.name === 'heading' && node.attrs.level === 1) {
                                h1Count++
                                if (h1Count > 1) {
                                    tr.setNodeAttribute(pos, 'level', 2)
                                }
                            }
                        })
                        return h1Count > 1 ? tr : null
                    },
                }),
            ]
        },
    })

    return useEditor({
        content: document?.content.document,
        editable: metadata.workflow.state === 'draft',
        extensions: [
            CustomDocument,
            StarterKit.configure({document: false}),
            Image,
            Underline,
            TextAlign.configure({
                types: ['heading', 'paragraph'],
            }),
            Placeholder.configure({
                showOnlyCurrent: false,
                placeholder: ({node}) => {
                    if (
                        // @ts-ignore
                        node.type.name === 'heading' && node.attrs.level === 1
                    ) {
                        return 'Title...'
                    }
                    return 'Type / to view options'
                },
            }),
            Commands.configure({
                suggestion: Suggestion,
            }),
        ],
        onUpdate: onUpdate,
        editorProps: {
            attributes: {
                class:
                    'flex flex-col focus:outline-none w-full h-full prose prose-sm prose-p:my-1',
            },
            handleDrop(view, event) {
                event.preventDefault()
                const fileList = event.dataTransfer?.files
                if (!fileList?.length) return false
                const files: File[] = []
                for (let i = 0; i < fileList.length; i++) {
                    const file = fileList.item(i)
                    if (file) {
                        files.push(file)
                    }
                }
                async function doUpload() {
                    toast({title: 'Uploading files, please wait...'})
                    try {
                        const metadataIds = await uploader.upload(files)
                        const coords = view.posAtCoords({
                            left: event.clientX,
                            top: event.clientY,
                        })
                        if (coords) {
                            for (const metadataId of metadataIds) {
                                const image = view.state.schema.nodes.image.create({
                                    src: '/content/image?id=' + metadataId,
                                    metadataId: metadataId,
                                })
                                view.dispatch(view.state.tr.insert(coords.pos, image))
                            }
                        }
                        toast({title: 'File(s) uploaded'})
                    } catch (e) {
                        toast({
                            title: 'Error uploading file(s)',
                            description: (e as unknown as any).message,
                        })
                    }
                }
                doUpload()
                return true
            },
        },
    })
}