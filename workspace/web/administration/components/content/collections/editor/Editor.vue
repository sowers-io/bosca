<script lang="ts" setup>
import {type Range} from '@tiptap/vue-3'
import {
  AttributeUiType,
  type CollectionFragment,
  type CollectionMetadataRelationshipFragment,
  type CollectionTemplateFragment,
  type DocumentTemplateAttribute,
  type ParentCollectionFragment,
} from '@/lib/graphql/graphql'
import {toast} from '~/components/ui/toast'
import {Uploader} from '@/lib/uploader'
import {hideAll} from 'tippy.js'
import {OpenMediaPickerEvent} from '@/lib/editor/commanditems'
import {AttributeState, newAttributeState} from '~/lib/attribute.ts'
import type {WatchSource} from 'vue'
import {
  Pagination,
  PaginationEllipsis,
  PaginationFirst,
  PaginationLast, PaginationList, PaginationListItem,
  PaginationNext, PaginationPrev
} from "~/components/ui/pagination";

const client = useBoscaClient()
const uploader = new Uploader(client)

const props = defineProps<{
  collection: CollectionFragment
  collectionCollection: ParentCollectionFragment | null | undefined
  parents: ParentCollectionFragment[]
  relationships: CollectionMetadataRelationshipFragment[]
  template: CollectionTemplateFragment | null
  hasChanges: boolean
}>()

const collection = defineModel('collection', {type: Object, default: null})
const hasChanges = defineModel('hasChanges', {type: Boolean, default: null})
const title = ref(props.collection.name)
const attributes = reactive(new Map<string, AttributeState>())

function isEqual(a: any, b: any) {
  if (!a && !b) return true
  if (a === b) return true
  return typeof a === 'string' && a.length === 0 && !b ||
      typeof b === 'string' && b.length === 0 && !a
}

function updateHasChanges() {
  if (title.value != props.collection.name) {
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
          if (!isEqual(attr.value, props.collection.attributes[attr.key])) {
            changes = true
          }
          break
        case AttributeUiType.Collection:
          if (props.parents.length != attr.value?.length) {
            changes = true
            break
          }
          const s = new Set()
          for (const c of props.parents) {
            s.add(c.id)
          }
          for (const c of attr.value) {
            if (!s.has(c.id)) {
              changes = true
              break
            }
          }
          break
        case AttributeUiType.Image:
        case AttributeUiType.File:
          const metadataId = toRaw(attr.value)?.metadata?.id
          const relationship = attr.configuration.relationship
          const currId = props.relationships.find((r) =>
              r.relationship === relationship
          )?.metadata?.id
          if (!isEqual(metadataId, currId)) {
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
  }, 500)
}

async function updateAttributes() {
  for (const attribute of props.template?.attributes || []) {
    let attr = attributes.get(attribute.key)
    if (!attr) {
      attr = newAttributeState(attribute as DocumentTemplateAttribute)
      attributes.set(attribute.key, reactive(attr) as AttributeState)
      const attrRef = attributes.get(attribute.key) as unknown as WatchSource<
          AttributeState
      >
      const key = attr.key
      const cfg = attr.configuration
      watch(attrRef, checkForChanges)
      switch (attr.ui) {
        case AttributeUiType.Textarea:
        case AttributeUiType.Input:
          attr.value = props.collection.attributes[key]
          break
        case AttributeUiType.Collection:
          if (attr.list) {
            attr.value = (props.parents || []).filter((c) => c.attributes.type === cfg.type)
          } else {
            attr.value = (props.parents || []).find((c) => c.attributes.type === cfg.type)
          }
          break
        case AttributeUiType.Image:
        case AttributeUiType.File:
          const r = props.relationships.find((r) =>
              r.relationship === attr?.configuration.relationship
          )
          if (r) {
            attr.value = {
              metadata: r.metadata,
              relationship: r.relationship,
            }
          }
          break
      }
    }
  }
}

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
      if (attribute.list) {
        attribute.value = []
      } else {
        attribute.value = null
      }
      break
  }
  const attr = props.template?.attributes?.find((a) => a.key === attribute.key)
  for (const workflow of attr?.workflows || []) {
    await client.collections.enqueueCollectionWorkflow(
        workflow.workflow!.id,
        props.collection.id,
    )
    if (workflows.length > 0) workflows += ', '
    workflows += workflow.workflow!.name
  }
  attribute.loading = true
  toast({title: 'Executing: ' + attribute.description})
}

function onOpenMediaPicker(event: OpenMediaPickerEvent) {
  hideAll()
  pendingRange = event.range
  mediaDialogOpen.value = true
}

async function onSave() {
  hasChanges.value = false
}

async function onReset() {
  attributes.clear()
  title.value = props.collection.name
  await updateAttributes()
  hasChanges.value = false
}

onMounted(async () => {
  await updateAttributes()
  window.addEventListener('save-collection', onSave)
  window.addEventListener('reset-collection', onReset)
  // @ts-ignore
  window.addEventListener(OpenMediaPickerEvent.NAME, onOpenMediaPicker)
})

onUpdated(() => updateAttributes())

onUnmounted(() => {
  window.removeEventListener('save-collection', onSave)
  window.removeEventListener('reset-collection', onReset)
  // @ts-ignore
  window.removeEventListener(OpenMediaPickerEvent.NAME, onOpenMediaPicker)
})

watch(collection, async () => {
  if (!hasChanges.value) {
    await onReset()
  }
  await updateAttributes()
})

watch(title, async () => {
  updateHasChanges()
})

const selectedTab = ref('metadata')
const currentPage = ref(1)
const limit = ref(18)
const offset = computed(() => (currentPage.value - 1) * limit.value)
const count = ref(0)

const editable = computed(() => props.collection.workflow.state === 'draft')
</script>

<template>
  <div class="w-full h-full" v-if="collection">
    <div class="grid grid-cols-3 gap-2 h-full w-full">
      <div class="col-span-2">
        <Input :disabled="collection.workflow.state !== 'draft'"
            v-model="title"
            class="w-full"
            placeholder="Title" />
        <Tabs
            v-model:model-value="selectedTab"
            class="h-full space-y-6 mt-4"
        >
          <div class="flex">
            <TabsList>
              <TabsTrigger value="metadata">
                Content
              </TabsTrigger>
              <TabsTrigger value="collections">
                Collections
              </TabsTrigger>
            </TabsList>
            <div class="grow"></div>
            <div class="flex items-center mr-4">
              <Pagination
                  v-slot="{ page }"
                  v-model:page="currentPage"
                  :total="count"
                  :items-per-page="limit"
                  :sibling-count="1"
                  show-edges
              >
                <PaginationList v-slot="{ items }" class="flex items-center gap-1">
                  <PaginationFirst />
                  <PaginationPrev />

                  <template v-for="(item, index) in items">
                    <PaginationListItem
                        v-if="item.type === 'page'"
                        :key="index"
                        :value="item.value"
                        as-child
                    >
                      <Button
                          class="w-10 h-10 p-0"
                          :variant="item.value === page ? 'default' : 'outline'"
                      >
                        {{ item.value }}
                      </Button>
                    </PaginationListItem>
                    <PaginationEllipsis v-else :key="item.type" :index="index" />
                  </template>

                  <PaginationNext />
                  <PaginationLast />
                </PaginationList>
              </Pagination>
            </div>
            <div class="flex items-center">
              <Button :disabled="hasChanges">
                <Icon name="i-lucide-plus"/>
              </Button>
            </div>
          </div>
          <TabsContent value="collections" class="border-none p-0 mt-0 outline-none">
            <ContentCollectionsEditorCollectionsList
                :collection="collection"
                :offset="offset"
                :limit="limit"
                v-model:count="count"
            />
          </TabsContent>
          <TabsContent value="metadata" class="border-none p-0 mt-0 outline-none">
            <ContentCollectionsEditorMetadataList
                :collection="collection"
                :offset="offset"
                :limit="limit"
                v-model:count="count"
            />
          </TabsContent>
        </Tabs>
      </div>
      <div class="min-h-[calc(100dvh-170px)]">
        <div class="bg-accent rounded-md px-4 py-2 h-full">
          <ContentEditorAttributes
              :parents="parents"
              :attributes="attributes"
              :workflows-enabled="!hasChanges"
              :uploader="uploader"
              :editable="editable"
              :on-run-workflow="onRunWorkflow"
          />
        </div>
      </div>
    </div>
  </div>
</template>

