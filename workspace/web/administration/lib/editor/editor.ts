import { useEditor } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import { Image } from '~/lib/editor/image.ts'
import Underline from '@tiptap/extension-underline'
import TextAlign from '@tiptap/extension-text-align'
import Link from '@tiptap/extension-link'
import Placeholder from '@tiptap/extension-placeholder'
import Commands from '~/lib/editor/commands.ts'
import Superscript from '@tiptap/extension-superscript'
import Suggestion from '~/lib/editor/suggestion.ts'
import { Bible } from '~/lib/editor/bible.ts'
import { toast } from '~/components/ui/toast'
import type {
  DocumentFragment,
  MetadataFragment,
  DocumentTemplateFragment,
} from '~/lib/graphql/graphql.ts'
import { Plugin } from '@tiptap/pm/state'
import Document from '@tiptap/extension-document'
import { type EditorEvents } from '@tiptap/core'
import {Container} from "~/lib/editor/container.ts";

export function newEditor(
  document: DocumentFragment,
  metadata: MetadataFragment,
  template: DocumentTemplateFragment | null = null,
  uploader: Uploader | null = null,
  onUpdate: ((props: EditorEvents['update']) => void) | null = null,
  editable: boolean = true,
) {
  const templateContent = template?.configuration?.content
    ? toRaw(template?.configuration?.content)
    : null
  const CustomDocument = Document.extend({
    content: templateContent || 'heading block+',
    addProseMirrorPlugins() {
      return [
        new Plugin({
          appendTransaction: (transactions, oldState, newState) => {
            const { doc, tr } = newState
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
  const content = document?.content?.document
    ? toRaw(document?.content?.document)
    : null
  return useEditor({
    content: content,
    editable: metadata.workflow.state === 'draft' && editable,
    extensions: [
      CustomDocument,
      StarterKit.configure({ document: false }),
      Container.configure({ metadata: metadata, containers: template?.containers || [] }),
      Image,
      Link,
      Underline,
      Superscript,
      TextAlign.configure({
        types: ['heading', 'paragraph'],
      }),
      Placeholder.configure({
        showOnlyCurrent: false,
        placeholder: ({ node }) => {
          if (node.type.name === 'heading' && node.attrs.level === 1) {
            return 'Title...'
          }
          return 'Type / to view options'
        },
      }),
      Commands.configure({
        suggestion: Suggestion,
      }),
      Bible,
    ],
    onContentError: (ev) => {
      console.error('content error', ev)
    },
    onUpdate: (ev) => {
      if (onUpdate) {
        onUpdate(ev)
      }
    },
    editorProps: {
      attributes: {
        class:
          'flex flex-col focus:outline-none w-full h-full prose prose-sm prose-p:my-1',
      },
      handleDrop(view, event) {
        event.preventDefault()
        const fileList = event.dataTransfer?.files
        if (!fileList || !fileList.length) return false
        const files: File[] = []
        for (let i = 0; i < fileList.length; i++) {
          const file = fileList.item(i)
          if (file) {
            files.push(file)
          }
        }
        async function doUpload() {
          if (!uploader) return
          toast({ title: 'Uploading files, please wait...' })
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
            toast({ title: 'File(s) uploaded' })
          } catch (e) {
            toast({
              title: 'Error uploading file(s)',
              description: (e as unknown as any).message,
            })
          }
        }
        doUpload().then((r) => {})
        return true
      },
    },
  })
}
