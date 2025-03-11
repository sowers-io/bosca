<script setup lang="ts">
import { toast } from '~/components/ui/toast'
import {
  type ParentCollectionFragment,
  WorkflowStateType,
} from '~/lib/graphql/graphql.ts'

const router = useRouter()
const breadcrumbs = useBreadcrumbs()
const route = useRoute()

const client = useBoscaClient()

const collection = ref(
  await client.collections.get(route.params.collectionId.toString()),
)
const relationships = ref(
  await client.collections.getMetadataRelationships(
    route.params.collectionId.toString(),
  ),
)
const parents = ref(
  await client.collections.getCollectionParents(
    route.params.collectionId.toString(),
  ),
)
const collectionCollection = computed(() => {
  return parents.value?.find((c) =>
    c.attributes && c.attributes['editor.type'] === 'Collection'
  ) as
    | ParentCollectionFragment
    | undefined
})
const parentCollections = computed(() => {
  return parents.value?.filter((c) =>
    !c.attributes || c.attributes['editor.type'] !== 'Document'
  ) || []
})
const template = ref(
  collection.value?.templateMetadata?.id &&
    collection.value.templateMetadata?.version
    ? await client.metadata.getCollectionTemplate(
      collection.value.templateMetadata.id,
      collection.value.templateMetadata.version,
    )
    : null,
)
const { data: states } = client.workflows.getStatesAsyncData()
const stateName = computed(() => {
  return states.value?.find((s) => s.id === collection.value?.workflow.state)
    ?.name
})
const hasChanges = ref(false)
const confirmDelete = ref(false)
const confirmReset = ref(false)
const outOfDate = ref(false)

function onSave() {
  window.dispatchEvent(new Event('save-collection'))
}

function reset() {
  confirmReset.value = true
}

function doReset() {
  outOfDate.value = false
  confirmReset.value = false
  window.dispatchEvent(new Event('reset-collection'))
}

async function onPublish() {
  if (collection.value?.workflow.pending) {
    return
  }
  const states = await client.workflows.getStates() || []
  const published =
    states.find((s) => s.type === WorkflowStateType.Published)?.id || ''
  if (collection.value!.workflow.state !== published) {
    await client.collections.beginTransition(
      collection.value!.id,
      published,
      'Publishing Collection',
    )
  }
  if (!collection.value!.public) {
    await client.metadata.setPublic(collection.value!.id, true)
  }
  for (const relationship of relationships.value || []) {
    if (relationship.metadata.workflow.pending) {
      continue
    }
    if (relationship.metadata.workflow.state !== published) {
      await client.metadata.beginTransition(
        relationship.metadata.id,
        relationship.metadata.version,
        published,
        'Publishing Document',
      )
    }
    if (!relationship.metadata.public) {
      await client.metadata.setPublic(relationship.metadata.id, true)
    }
    if (!relationship.metadata.publicContent) {
      await client.metadata.setContentPublic(relationship.metadata.id, true)
    }
  }
}

async function onUnpublish() {
  const states = await client.workflows.getStates() || []
  await client.collections.beginTransition(
    collection.value!.id,
    states.find((s) => s.type === WorkflowStateType.Draft)?.id || '',
    'Unpublishing Collection',
  )
}

async function onPreview() {
  const configuration = await client.configurations.getConfiguration(
    'preview.url',
  )
  if (!configuration || !collection.value?.slug) return
  window.open(
    configuration.value.value + '?slug=' + collection.value!.slug,
    '_blank',
  )
}

function onDelete() {
  confirmDelete.value = true
}

async function doDelete() {
  confirmDelete.value = false
  await client.collections.delete(collection.value!.id)
  await router.push('/collections')
}

client.listeners.onCollectionChanged(async (id) => {
  if (id === collection.value?.id) {
    try {
      parents.value = await client.collections.getCollectionParents(id)
      relationships.value = await client.collections.getMetadataRelationships(
        id,
      )
      collection.value = await client.collections.get(id)
      outOfDate.value = hasChanges.value
      toast({ title: 'Collection updated.' })
    } catch (ignore) {
    }
  }
})

onMounted(() => {
  breadcrumbs.set([
    { title: 'Collections', to: '/collections' },
    { title: 'Edit Collection' },
  ])
})
</script>
<template>
  <div>
    <div class="flex items-center">
      <div class="flex">
        <div>
          <Badge variant="secondary">{{ stateName }}
            <span v-if="collection?.workflow?.pending">*</span>
          </Badge>
          <Badge v-if="hasChanges" variant="outline" class="ms-2 text-gray-400"
          >Has Changes</Badge>
          <Badge v-if="outOfDate" variant="destructive" class="ms-2"
          >Out of Date</Badge>
        </div>
      </div>
      <div class="grow"></div>
      <div class="flex gap-2">
        <Tooltip v-if="collection?.workflow?.state === 'draft'">
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
        <Tooltip v-if="collection?.workflow?.state === 'draft'">
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
            <p>Save Collection</p>
          </TooltipContent>
        </Tooltip>
        <Tooltip>
          <TooltipTrigger as-child>
            <Button
              @click="onPreview"
              :disabled="hasChanges"
              class="flex gap-2"
              variant="secondary"
            >
              <Icon name="i-lucide-screen-share" class="size-4" />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>Preview Collection</p>
          </TooltipContent>
        </Tooltip>
        <Tooltip>
          <TooltipTrigger as-child>
            <Button @click="onDelete" class="flex gap-2" variant="secondary">
              <Icon name="i-lucide-trash" class="size-4" />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>Delete Collection</p>
          </TooltipContent>
        </Tooltip>
        <Tooltip v-if="collection?.workflow?.state === 'draft'">
          <TooltipTrigger as-child>
            <Button
              @click="onPublish"
              :disabled="hasChanges || collection?.workflow?.pending"
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
        <Tooltip v-if="collection?.workflow?.state === 'published'">
          <TooltipTrigger as-child>
            <Button
              @click="onUnpublish"
              :disabled="collection?.workflow?.pending"
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
        <Tooltip>
          <TooltipTrigger as-child>
            <Button
              :disabled="hasChanges"
              @click="
                router.push(
                  '/collections/edit/' + collection!.id +
                    '?collection=true',
                )
              "
              class="flex gap-2"
              variant="secondary"
            >
              <Icon name="i-lucide-bolt" class="size-4" />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>Advanced Settings</p>
          </TooltipContent>
        </Tooltip>
      </div>
    </div>
    <div class="border-none p-0 outline-none mt-4">
      <ContentCollectionsEditor
        :parents="parentCollections"
        :collectionCollection="collectionCollection"
        :relationships="relationships || []"
        :template="template"
        v-model:collection="collection"
        v-model:has-changes="hasChanges"
      />
    </div>
    <Dialog v-model:open="confirmDelete">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Delete Collection</DialogTitle>
          <DialogDescription>
            Are you sure you want to delete this collection?<br />
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <div class="flex w-full items-center gap-4">
            <div class="flex text-sm text-gray-400">This cannot be undone.</div>
            <div class="grow"></div>
            <Button type="button" variant="destructive" @click="doDelete">
              Delete Collection
            </Button>
          </div>
        </DialogFooter>
      </DialogContent>
    </Dialog>
    <Dialog v-model:open="confirmReset">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Reset Collection</DialogTitle>
          <DialogDescription>
            Are you sure you want to reset this collection?<br />
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <div class="flex w-full items-center gap-4">
            <div class="flex text-sm text-gray-400">This cannot be undone.</div>
            <div class="grow"></div>
            <Button type="button" variant="destructive" @click="doReset">
              Reset Collection
            </Button>
          </div>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>
