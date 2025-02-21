<script setup lang="ts">
import { toast } from '~/components/ui/toast'
import {
  type ParentCollectionFragment,
  WorkflowStateType,
} from '~/lib/graphql/graphql.ts'

const router = useRouter()
const breadcrumbs = useBreadcrumbs()
const route = useRoute()
const selectedItem = ref('document')

const client = useBoscaClient()

const metadata = ref(
  await client.metadata.get(route.params.metadataId.toString()),
)
const relationships = ref(
  await client.metadata.getRelationships(route.params.metadataId.toString()),
)
const parents = ref(
  await client.metadata.getParents(route.params.metadataId.toString()),
)
const document = ref(
  await client.metadata.getDocument(route.params.metadataId.toString()),
)
const documentCollection = computed(() => {
  return parents.value?.find((c) => c.attributes['editor.type']) as
    | ParentCollectionFragment
    | undefined
})
const parentCollections = computed(() => {
  return parents.value?.filter((c) => !c.attributes['editor.type']) || []
})
const template = ref(
  document.value.templateMetadataId && document.value.templateMetadataVersion
    ? await client.metadata.getDocumentTemplate(
      document.value.templateMetadataId,
      document.value.templateMetadataVersion,
    )
    : null,
)
const { data: states } = client.workflows.getStatesAsyncData()
const stateName = computed(() => {
  return states.value?.find((s) => s.id === metadata.value.workflow.state)?.name
})
const hasChanges = ref(false)
const confirmDelete = ref(false)
const confirmReset = ref(false)
const outOfDate = ref(false)

function onSave() {
  window.dispatchEvent(new Event('save-document'))
}

function reset() {
  confirmReset.value = true
}

function doReset() {
  outOfDate.value = false
  window.dispatchEvent(new Event('reset-document'))
  confirmReset.value = false
}

async function onPublish() {
  const states = await client.workflows.getStates() || []
  await client.metadata.beginTransition(
    metadata.value.id,
    metadata.value.version,
    states.find((s) => s.type === WorkflowStateType.Published)?.id || '',
    'Publishing Document',
  )
}

async function onUnpublish() {
  const states = await client.workflows.getStates() || []
  await client.metadata.beginTransition(
    metadata.value.id,
    metadata.value.version,
    states.find((s) => s.type === WorkflowStateType.Draft)?.id || '',
    'Unpublishing Document',
  )
}

function onPreview() {
  toast({ title: 'Preview not implemented yet.' })
}

function onDelete() {
  confirmDelete.value = true
}

async function doDelete() {
  confirmDelete.value = false
  await client.metadata.delete(metadata.value.id)
  await router.push('/content')
}

client.listeners.onMetadataChanged(async (id) => {
  if (id === metadata.value.id) {
    try {
      document.value = await client.metadata.getDocument(id)
      parents.value = await client.metadata.getParents(id)
      relationships.value = await client.metadata.getRelationships(id)
      metadata.value = await client.metadata.get(id)
      outOfDate.value = hasChanges.value
      toast({ title: 'Document updated.' })
    } catch (ignore) {}
  }
})

onMounted(() => {
  breadcrumbs.set([
    { title: 'Content', to: '/content' },
    { title: 'Edit Document' },
  ])
})
</script>
<template>
  <div>
    <Tabs v-model:model-value="selectedItem" class="h-full space-y-6">
      <div class="flex items-center">
        <TabsList>
          <TabsTrigger value="document">
            Document
          </TabsTrigger>
          <TabsTrigger value="metadata">
            Metadata
          </TabsTrigger>
        </TabsList>
        <div class="grow items-center justify-center flex">
          <div>
            <Badge variant="secondary">{{ stateName }}<span
                v-if="metadata.workflow.pending"
              >*</span></Badge>
            <Badge
              v-if="hasChanges"
              variant="outline"
              class="ms-2 text-gray-400"
            >Has Changes</Badge>
            <Badge v-if="outOfDate" variant="destructive" class="ms-2"
            >Out of Date</Badge>
          </div>
        </div>
        <div
          v-if="selectedItem === 'document'"
          class="flex gap-2"
        >
          <Tooltip v-if="metadata?.workflow?.state === 'draft'">
            <TooltipTrigger as-child>
              <Button
                @click="reset"
                class="flex gap-2"
                variant="secondary"
                :disabled="!hasChanges"
              >
                <Icon name="i-lucide-rotate-ccw" class="size-4" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <p>Reset Document</p>
            </TooltipContent>
          </Tooltip>
          <Tooltip v-if="metadata?.workflow?.state === 'draft'">
            <TooltipTrigger as-child>
              <Button
                @click="onSave"
                class="flex gap-2"
                variant="secondary"
                :disabled="!hasChanges"
              >
                <Icon name="i-lucide-save" class="size-4" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <p>Save Document</p>
            </TooltipContent>
          </Tooltip>
          <Tooltip>
            <TooltipTrigger as-child>
              <Button @click="onPreview" class="flex gap-2" variant="secondary">
                <Icon name="i-lucide-screen-share" class="size-4" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <p>Preview Document</p>
            </TooltipContent>
          </Tooltip>
          <Tooltip>
            <TooltipTrigger as-child>
              <Button @click="onDelete" class="flex gap-2" variant="secondary">
                <Icon name="i-lucide-trash" class="size-4" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <p>Delete Document</p>
            </TooltipContent>
          </Tooltip>
          <Tooltip v-if="metadata?.workflow?.state === 'draft'">
            <TooltipTrigger as-child>
              <Button
                @click="onPublish"
                :disabled="hasChanges || metadata?.workflow?.pending"
                class="flex gap-2"
                variant="secondary"
              >
                <Icon name="i-lucide-square-play" class="size-4" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <p>Publish Document</p>
            </TooltipContent>
          </Tooltip>
          <Tooltip v-if="metadata?.workflow?.state === 'published'">
            <TooltipTrigger as-child>
              <Button
                @click="onUnpublish"
                :disabled="metadata?.workflow?.pending"
                class="flex gap-2"
                variant="secondary"
              >
                <Icon name="i-lucide-square-square" class="size-4" />
              </Button>
            </TooltipTrigger>
            <TooltipContent>
              <p>Unpublish Document</p>
            </TooltipContent>
          </Tooltip>
        </div>
      </div>
      <TabsContent
        value="document"
        class="border-none p-0 outline-none"
        force-mount
      >
        <ContentMetadataEditor
          :documentCollection="documentCollection"
          :parents="parentCollections"
          :relationships="relationships"
          :document="document"
          :template="template"
          v-model:metadata="metadata"
          v-model:has-changes="hasChanges"
        />
      </TabsContent>
      <TabsContent
        value="metadata"
        class="border-none p-0 outline-none"
        force-mount
      >
        <ContentMetadata :metadata-id="route.params.metadataId.toString()" />
      </TabsContent>
    </Tabs>
    <Dialog v-model:open="confirmDelete">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Delete Document</DialogTitle>
          <DialogDescription>
            Are you sure you want to delete this document?<br />
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <div class="flex w-full items-center gap-4">
            <div class="flex text-sm text-gray-400">This cannot be undone.</div>
            <div class="grow"></div>
            <Button type="button" variant="destructive" @click="doDelete">
              Delete Document
            </Button>
          </div>
        </DialogFooter>
      </DialogContent>
    </Dialog>
    <Dialog v-model:open="confirmReset">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Reset Document</DialogTitle>
          <DialogDescription>
            Are you sure you want to reset this document?<br />
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <div class="flex w-full items-center gap-4">
            <div class="flex text-sm text-gray-400">This cannot be undone.</div>
            <div class="grow"></div>
            <Button type="button" variant="destructive" @click="doReset">
              Reset Document
            </Button>
          </div>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>
