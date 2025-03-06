<script lang="ts" setup>
import {
  type DocumentTemplateContainerFragment,
} from '~/lib/graphql/graphql.js'
import { NodeViewContent, NodeViewWrapper } from '@tiptap/vue-3'
import type { Editor } from '@tiptap/core'
import { hideAll } from 'tippy.js'
import { toast } from '~/components/ui/toast'

const props = defineProps<{
  name: String
  containers: DocumentTemplateContainerFragment[]
  extension: any
  editor: Editor
  getPos: Function
  node: any
  HTMLAttributes: Record<string, any>
}>()

const client = useBoscaClient()
const loading = ref(false)

const nodeContainer = computed(() => {
  return props.extension.options.containers?.find((c: any) =>
    c.id === props.HTMLAttributes.name
  )
})

async function onRunWorkflow() {
  if (loading.value || !nodeContainer.value) return
  try {
    loading.value = true
    hideAll()
    let workflows = ''
    for (const workflow of nodeContainer.value?.workflows || []) {
      await client.workflows.enqueueMetadataWorkflow(
        workflow.workflow!.id,
        props.extension.options.metadata.id,
        props.extension.options.metadata.version,
      )
      if (workflows.length > 0) workflows += ', '
      workflows += workflow.workflow!.name
    }
    toast({ title: 'Executing: ' + nodeContainer.value.description })
  } catch (e) {
    loading.value = false
    toast({
      title: 'Error Executing: ' + nodeContainer.value.description + ': ' +
        (e as unknown as any).message,
    })
  }
}

client.listeners.onMetadataSupplementaryChanged(async (id, key) => {
  if (id === props.extension.options.metadata.id) {
    if (key === nodeContainer.value?.supplementaryKey) {
      const pos = props.getPos?.()
      if (typeof pos !== 'number') {
        console.error('Could not determine position of the node')
        loading.value = false
        return
      }
      try {
        const content = await client.metadata.getSupplementaryJson(id, key)
        if (content?.content && content.content.length > 0) {
          const nodeContent = props.editor.schema.nodeFromJSON(
            content.content[0],
          )
          props.editor.view.dispatch(
            props.editor.state.tr.replaceRangeWith(
              pos + 1,
              pos + props.node.content.size + 1,
              nodeContent,
            ),
          )
        }
      } finally {
        loading.value = false
      }
    }
  }
})
</script>

<template>
  <NodeViewWrapper>
    <div class="container">
      <div class="container-name flex items-center justify-between">
        {{ nodeContainer?.name || HTMLAttributes.name }}
        <template
          v-if="
            nodeContainer?.workflows &&
            (nodeContainer?.workflows?.length || 0) > 0
          "
        >
          <Button
            class="flex items-center justify-center ms-2 size-8 p-0"
            variant="ghost"
            @click="onRunWorkflow()"
          >
            <Icon
              name="i-lucide-sparkles"
              class="size-4"
              v-if="!loading"
            />
            <Icon
              name="i-lucide-loader-circle"
              class="size-4 animate-spin"
              v-else
            />
          </Button>
        </template>
      </div>
      <div class="container-content">
        <NodeViewContent />
      </div>
    </div>
  </NodeViewWrapper>
</template>
