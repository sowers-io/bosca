import { VueRenderer } from '@tiptap/vue-3'
import type { Editor } from '@tiptap/core'
import tippy, { type GetReferenceClientRect, type Instance } from 'tippy.js'

import CommandsList from '@/components/content/metadata/editor/CommandList.vue'
import { CommandItems } from '~/lib/editor/commanditems'

export default {
  items: ({ query }: { query: string }) => {
    return CommandItems.filter((item) =>
      item.label.toLowerCase().startsWith(query.toLowerCase())
    ).slice(0, 10)
  },

  render: () => {
    let component: VueRenderer | null = null
    let popup: Instance[] = []

    return {
      onStart: (
        props: { editor: Editor; clientRect: GetReferenceClientRect | null },
      ) => {
        const { state } = props.editor.view
        const { $from } = state.selection

        if (
          $from.parent.type.name === 'heading' && $from.parent.attrs.level === 1
        ) {
          component = null
          popup = []
          return
        }

        component = new VueRenderer(CommandsList, {
          props,
          editor: props.editor,
        })

        if (!props.clientRect) {
          return
        }

        // @ts-ignore: this works fine
        popup = tippy('body', {
          getReferenceClientRect: props.clientRect,
          appendTo: () => document.body,
          content: component.element,
          showOnCreate: true,
          interactive: true,
          trigger: 'manual',
          placement: 'bottom-start',
        })
      },

      onUpdate(
        props: { editor: Editor; clientRect: GetReferenceClientRect | null },
      ) {
        if (popup.length === 0 || !component) return

        component.updateProps(props)

        if (!props.clientRect) {
          return
        }

        popup[0].setProps({
          getReferenceClientRect: props.clientRect,
        })
      },

      onKeyDown(
        props: {
          editor: Editor
          event: KeyboardEvent
          clientRect: GetReferenceClientRect | null
        },
      ) {
        if (popup.length === 0) return
        if (props.event.key === 'Escape') {
          popup[0].hide()

          return true
        }

        return component?.ref?.onKeyDown(props)
      },

      onExit() {
        if (popup.length !== 0) {
          popup[0].destroy()
        }
        if (component) {
          component.destroy()
        }
      },
    }
  },
}
