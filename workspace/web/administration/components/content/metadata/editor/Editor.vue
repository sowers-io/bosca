<script lang="ts" setup>
import {BubbleMenu, EditorContent, type Range} from '@tiptap/vue-3'
import {
  AttributeUiType,
  type CollectionIdNameFragment,
  type DocumentFragment,
  type DocumentTemplateAttribute,
  type DocumentTemplateFragment,
  type MetadataFragment,
  type MetadataRelationshipFragment,
  type ParentCollectionFragment,
} from '@/lib/graphql/graphql'
import {toast} from '~/components/ui/toast'
import {Uploader} from '@/lib/uploader'
import {hideAll} from 'tippy.js'
import {CommandItems, OpenMediaPickerEvent} from '@/lib/editor/commanditems'
import {AttributeState, newAttributeState} from "~/lib/attribute.ts";
import {save} from "~/lib/editor/save.ts";
import {newEditor} from "~/lib/editor/editor.ts";
import type {WatchSource} from "vue";

const client = useBoscaClient()
const uploader = new Uploader(client)

const props = defineProps<{
  metadata: MetadataFragment
  documentCollection: ParentCollectionFragment
  parents: CollectionIdNameFragment[]
  document: DocumentFragment
  template: DocumentTemplateFragment
  hasChanges: boolean
}>()

const metadata = defineModel('metadata', {type: Object, default: null})
const hasChanges = defineModel('hasChanges', {type: Boolean, default: null})
const hasDocChanges = ref(false)
const relationships: Ref<Array<MetadataRelationshipFragment>> = ref([])
const title = ref(props.metadata.name)
const attributes = reactive(new Map<string, AttributeState>())

function isEqual(a: any, b: any) {
  if (!a && !b) return true
  if (a === b) return true
  return typeof a === 'string' && a.length === 0 && !b ||
      typeof b === 'string' && b.length === 0 && !a;
}

function updateHasChanges() {
  if (hasDocChanges.value) {
    hasChanges.value = true
    return
  }
  let changes = false
  for (const attribute of props.template?.attributes || []) {
    const attr = attributes.get(attribute.key)
    if (attr) {
      switch (attr.ui) {
        case AttributeUiType.Textarea:
        case AttributeUiType.Input:
          if (!isEqual(attr.value, props.metadata.attributes[attr.key])) {
            changes = true
          }
          break
        case AttributeUiType.Profile:
          if (!isEqual(attr.value?.profile?.id, props.metadata.profiles.find((p) => p.relationship === attr.key)?.profile?.id)) {
            changes = true
          }
          break
        case AttributeUiType.Collection:

          break
        case AttributeUiType.Image:
        case AttributeUiType.File:
          if (!isEqual(attr.value?.metadata?.id, relationships.value.find((r) => r.relationship === attr.key)?.metadata?.id)) {
            changes = true
          }
          break
      }
    }
    if (changes) break
  }
  hasChanges.value = changes
}

let debounceUpdate: any = null

function checkForChanges() {
  if (debounceUpdate) clearTimeout(debounceUpdate)
  debounceUpdate = setTimeout(() => {
    updateHasChanges()
  }, 1500)
}

async function updateAttributes() {
  if (editor.value) {
    const editable = props.metadata.workflow.state === 'draft'
    if (editable !== editor.value.isEditable) {
      editor.value.setEditable(editable, true)
    }
  }
  for (const attribute of props.template?.attributes || []) {
    let attr = attributes.get(attribute.key)
    if (!attr) {
      attr = newAttributeState(attribute as DocumentTemplateAttribute)
      attributes.set(attribute.key, reactive(attr) as AttributeState)
      const attrRef = attributes.get(attribute.key) as unknown as WatchSource<AttributeState>
      const key = attr.key
      watch(attrRef, checkForChanges)
      switch (attr.ui) {
        case AttributeUiType.Textarea:
        case AttributeUiType.Input:
          attr.value = props.metadata.attributes[key]
          break
        case AttributeUiType.Profile:
          attr.value = props.metadata.profiles.find((p) => p.relationship === key)
          break
        case AttributeUiType.Collection:
          attr.value = props.parents || []
          break
        case AttributeUiType.Image:
        case AttributeUiType.File:
          attr.value = relationships.value.find((r) => r.relationship === key)
          break
      }
    }
  }
}

const editor = newEditor(
    props.document,
    props.template,
    props.metadata,
    uploader,
    ({editor, transaction}) => {
      if (transaction.docChanged) {
        hasDocChanges.value = true
        hasChanges.value = true
      }
      const node = editor.view.dom.childNodes[0]
      title.value = node ? (node as HTMLElement)?.innerText : ''
    }
)

let pendingRange: Range | null | undefined = null
const mediaDialogOpen = ref(false)

async function onRunWorkflow(attribute: AttributeState) {
  if (attribute.loading) return
  hideAll()
  let workflows = ''
  switch (attribute.ui) {
    case AttributeUiType.Textarea:
    case AttributeUiType.Input:
      attribute.value = ''
      break
    case AttributeUiType.Collection:
      attribute.value = []
      break
  }
  const attr = props.template?.attributes?.find(a => a.key === attribute.key)
  for (const workflow of attr?.workflows || []) {
    await client.workflows.enqueueMetadataWorkflow(
        workflow.workflow!.id,
        props.metadata.id,
        props.metadata.version,
    )
    if (workflows.length > 0) workflows += ', '
    workflows += workflow.workflow!.name
  }
  attribute.loading = true
  toast({title: 'Executing: ' + attribute.description})
}

async function onAddMedia(id: string) {
  const e = editor.value
  if (!e) return
  let chain = e.chain().focus()
  if (pendingRange) {
    chain = chain.deleteRange(pendingRange)
    pendingRange = null
  }
  chain
      .setImage({
        src: '/content/image?id=' + id,
        metadataId: id,
      })
      .setTextSelection(e.state.selection.to + 1)
      .insertContent({type: 'paragraph'})
      .run()
  mediaDialogOpen.value = false
}

function onOpenMediaPicker(event: OpenMediaPickerEvent) {
  hideAll()
  pendingRange = event.range
  mediaDialogOpen.value = true
}

async function onSave() {
  hasDocChanges.value = false
  hasChanges.value = false
  const e = editor.value
  if (!e || !props.document) return
  await save(
      client,
      props.document,
      props.metadata,
      props.template,
      props.parents,
      title.value,
      relationships.value,
      attributes,
      e.state.doc.toJSON(),
  )
}

onMounted(async () => {
  relationships.value = await client.metadata.getRelationships(props.metadata.id)
  await updateAttributes()
  window.addEventListener('save-document', onSave)
  // @ts-ignore
  window.addEventListener(OpenMediaPickerEvent.NAME, onOpenMediaPicker)
})

onUpdated(() => updateAttributes())

onUnmounted(() => {
  window.removeEventListener('save-document', onSave)
  // @ts-ignore
  window.removeEventListener(OpenMediaPickerEvent.NAME, onOpenMediaPicker)
})

watch(metadata, async () => {
  relationships.value = await client.metadata.getRelationships(
      props.metadata.id,
  )
  await updateAttributes()
})

client.listeners.onMetadataSupplementaryChanged(async (id, key) => {
  console.log(id, key)
  const m = metadata.value as MetadataFragment
  if (id === m?.id) {
    for (const attr of props.template?.attributes || []) {
      if (attr.supplementaryKey !== key) continue
      const attribute = attributes.get(attr.key)
      if (!attribute) continue
      const wasLoading = attribute.loading
      attribute.setSupplementaryValue(client, id, key)
      if (wasLoading && !attribute.loading) {
        toast({title: 'Finished: ' + attr.description})
      }
      break
    }
  }
})

const editable = computed(() => props.metadata.workflow.state === 'draft')
</script>

<template>
  <div class="w-full h-full" v-if="editor">
    <bubble-menu
        class="flex border bg-background gap-1 rounded-md p-1 drop-shadow-xl ms-2 w-442px"
        :tippy-options="{ duration: 100, offset: [0, 20] }"
        :editor="editor"
    >
      <div class="flex items-center space-x-2">
        <button
            v-for="(item, index) in CommandItems"
            :key="index.toString() + '-' + item.name"
            :class="{
              'items-center justify-center inline-flex size-8 rounded-md': true,
              'hover:bg-accent hover:text-accent-foreground': true,
              'bg-accent text-foreground': item.name &&
                  editor.isActive(item.name, item.attributes) ||
                !item.name && item.attributes &&
                  editor.isActive(item.attributes || {}),
            }"
            @click="item.command({ editor })"
        >
          <Icon :name="item.icon" class="h-4 w-4"/>
        </button>
      </div>
    </bubble-menu>

    <div class="grid grid-cols-3 gap-2 h-full w-full">
      <editor-content class="col-span-2" :editor="editor"/>
      <div class="min-h-[calc(100dvh-170px)]">
        <div class="bg-accent rounded-md px-4 py-2 h-full">
          <div v-for="attr in template?.attributes || []" :key="attr.key">
            <template v-if="attr.ui === AttributeUiType.Collection">
              <ContentMetadataEditorCollectionAttribute
                  :collections="parents || []"
                  :attribute="attributes.get(attr.key) as AttributeState"
                  :editable="editable"
                  :workflows-enabled="!hasDocChanges"
                  :on-run-workflow="onRunWorkflow"
              />
            </template>
            <template v-if="attr.ui === AttributeUiType.Input">
              <ContentMetadataEditorInputAttribute
                  :attribute="attributes.get(attr.key) as AttributeState"
                  :editable="editable"
                  :workflows-enabled="!hasDocChanges"
                  :on-run-workflow="onRunWorkflow"
              />
            </template>
            <template v-if="attr.ui === AttributeUiType.Textarea">
              <ContentMetadataEditorTextAreaAttribute
                  :attribute="attributes.get(attr.key) as AttributeState"
                  :editable="editable"
                  :workflows-enabled="!hasDocChanges"
                  :on-run-workflow="onRunWorkflow"
              />
            </template>
            <template v-if="attr.ui === AttributeUiType.Image">
              <ContentMetadataEditorImageAttribute
                  :attribute="attributes.get(attr.key) as AttributeState"
                  :editable="editable"
                  :uploader="uploader"
              />
            </template>
            <template v-if="attr.ui === AttributeUiType.File">
              <ContentMetadataEditorFileAttribute
                  :attribute="attributes.get(attr.key) as AttributeState"
                  :editable="editable"
                  :uploader="uploader"
              />
            </template>
            <template v-if="attr.ui === AttributeUiType.Profile">
              <ContentMetadataEditorProfileAttribute
                  :attribute="attributes.get(attr.key) as AttributeState"
                  :editable="editable"
              />
            </template>
          </div>
        </div>
      </div>
    </div>
    <Dialog v-model:open="mediaDialogOpen">
      <DialogContent
          class="h-[calc(100dvh-100px)] w-[calc(100dvw-100px)] max-w-full overflow-y-auto"
      >
        <div class="flex flex-col gap-2 h-full">
          <h1 class="font-bold">Click to Select Your Item</h1>
          <ContentMedia :filter="{ mp4: true, webm: true, jpg: true, mp3: true, png: true, webp: true }"
                        :on-selected="onAddMedia"/>
        </div>
      </DialogContent>
    </Dialog>
  </div>
</template>

<style>
.tiptap {
  .is-empty:first-child::before, .is-empty:last-child::before {
    @apply float-left h-0 text-gray-300 dark:text-gray-800 pointer-events-none;
    content: attr(data-placeholder);
  }
}

.tiptap.ProseMirror {
  @apply border rounded-md px-4 w-full max-w-full;
}
</style>
