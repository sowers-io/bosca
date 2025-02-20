import type { Editor } from '@tiptap/core'
import type { Range } from '@tiptap/vue-3'

export class OpenMediaPickerEvent extends Event {
  constructor(readonly editor: Editor, readonly range: Range | undefined) {
    super(OpenMediaPickerEvent.NAME)
  }

  static readonly NAME = 'openMediaPicker'
}

export interface CommandProperties {
  editor: Editor
  range?: Range
}

export interface CommandItem {
  label: string
  name?: string | null
  attributes?: Record<string, string>
  icon: string
  command: (cmd: CommandProperties) => void
}

function newCommand(editor: Editor, range: Range | undefined) {
  const chain = editor.chain().focus()
  if (range) {
    chain.deleteRange(range)
  }
  return chain
}

export const CommandItems = [
  {
    name: 'heading',
    label: 'Heading 2',
    attributes: { level: 2 },
    icon: 'i-lucide-heading-2',
    command: ({ editor, range }: CommandProperties) => {
      if (!editor.isActive('heading', { level: 2 })) {
        newCommand(editor, range).setNode('heading', { level: 2 }).run()
      } else {
        newCommand(editor, range).setNode('paragraph').run()
      }
    },
  },
  {
    name: 'heading',
    label: 'Heading 3',
    attributes: { level: 3 },
    icon: 'i-lucide-heading-3',
    command: ({ editor, range }: CommandProperties) => {
      if (!editor.isActive('heading', { level: 3 })) {
        newCommand(editor, range).setNode('heading', { level: 3 }).run()
      } else {
        newCommand(editor, range).setNode('paragraph').run()
      }
    },
  },
  {
    name: 'bold',
    label: 'Bold',
    icon: 'i-lucide-bold',
    command: ({ editor, range }) =>
      newCommand(editor, range).toggleBold().run(),
  },
  {
    name: 'italic',
    label: 'Italic',
    icon: 'i-lucide-italic',
    command: ({ editor, range }) =>
      newCommand(editor, range).toggleItalic().run(),
  },
  {
    name: 'strike',
    label: 'Strike',
    icon: 'i-lucide-strikethrough',
    command: ({ editor, range }) =>
      newCommand(editor, range).toggleStrike().run(),
  },
  {
    label: 'Add Image',
    icon: 'i-lucide-image',
    command: ({ editor, range }) => {
      globalThis.dispatchEvent(new OpenMediaPickerEvent(editor, range))
    },
  },
] as CommandItem[]
