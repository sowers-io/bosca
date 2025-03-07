<script setup lang="ts">
import {
  type DocumentFragment,
  type DocumentTemplateFragment,
  type MetadataFragment,
  type MetadataRelationshipFragment,
  type ParentCollectionFragment,
  WorkflowStateType,
} from '~/lib/graphql/graphql.ts'

const router = useRouter()

const client = useBoscaClient()

const props = defineProps<{
  metadata: MetadataFragment
  relationships: Array<MetadataRelationshipFragment>
  parents: Array<ParentCollectionFragment>
  document: DocumentFragment
  documentTemplate: DocumentTemplateFragment | null | undefined
}>()

const metadata = defineModel<MetadataFragment>('metadata', {
  type: Object,
  default: null,
})

const documentCollection = computed(() => {
  return props.parents?.find((c) =>
    c.attributes['editor.type'] === 'Document'
  ) as
    | ParentCollectionFragment
    | undefined
})
const parentCollections = computed(() => {
  return props.parents?.filter((c) =>
    c.attributes['editor.type'] !== 'Document'
  ) || []
})
const { data: states } = client.workflows.getStatesAsyncData()
const stateName = computed(() => {
  return states.value?.find((s) => s.id === props.metadata.workflow.state)?.name
})
const pendingStateName = computed(() => {
  return states.value?.find((s) => s.id === props.metadata.workflow.pending)
    ?.name
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
  confirmReset.value = false
  window.dispatchEvent(new Event('reset-document'))
}

async function onPublish() {
  if (props.metadata.workflow.pending) {
    return
  }
  const states = await client.workflows.getStates() || []
  const published =
    states.find((s) => s.type === WorkflowStateType.Published)?.id || ''
  let stateValid: Date | null = null
  if (props.metadata.attributes['published']) {
    stateValid = new Date(Date.parse(props.metadata.attributes['published']))
  }
  if (props.metadata.workflow.state !== published) {
    await client.metadata.beginTransition(
      props.metadata.id,
      props.metadata.version,
      published,
      'Publishing Document',
      stateValid,
    )
  }
  if (!props.metadata.public) {
    await client.metadata.setPublic(props.metadata.id, true)
  }
  if (!props.metadata.publicContent) {
    await client.metadata.setContentPublic(props.metadata.id, true)
  }
  for (const relationship of props.relationships) {
    if (relationship.metadata.workflow.pending) {
      continue
    }
    if (relationship.metadata.workflow.state !== published) {
      await client.metadata.beginTransition(
        relationship.metadata.id,
        relationship.metadata.version,
        published,
        'Publishing Document',
        stateValid,
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
  if (props.metadata.workflow.stateValid) {
    await client.metadata.cancelTransition(
      props.metadata.id,
      props.metadata.version,
    )
  } else {
    await client.metadata.beginTransition(
      props.metadata.id,
      props.metadata.version,
      states.find((s) => s.type === WorkflowStateType.Draft)?.id || '',
      'Unpublishing Document',
    )
  }
}

async function onPreview() {
  const configuration = await client.configurations.getConfiguration(
    'preview.url',
  )
  if (!configuration || !props.metadata?.slug) return
  window.open(
    configuration.value.value + '?slug=' + props.metadata!.slug,
    '_blank',
  )
}

function onDelete() {
  confirmDelete.value = true
}

async function doDelete() {
  confirmDelete.value = false
  await client.metadata.delete(props.metadata.id)
  await router.push('/content')
}
</script>
<template>
  <div>
    <div class="flex items-center">
      <div class="flex">
        <div>
          <Badge variant="secondary">{{ stateName }}
            <span v-if="metadata.workflow.pending">*</span>
          </Badge>
          <Badge variant="secondary" class="ms-4" v-if="pendingStateName">{{
            pendingStateName
          }}</Badge>
          <Badge v-if="hasChanges" variant="outline" class="ms-2 text-gray-400"
          >Has Changes</Badge>
          <Badge v-if="outOfDate" variant="destructive" class="ms-2"
          >Out of Date</Badge>
        </div>
      </div>
      <div class="grow"></div>
      <div class="flex gap-2">
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
        <Tooltip
          v-if="
            metadata?.workflow?.state === 'draft' &&
            !metadata?.workflow?.stateValid
          "
        >
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
        <Tooltip
          v-if="
            metadata?.workflow?.state === 'published' ||
            metadata?.workflow?.stateValid
          "
        >
          <TooltipTrigger as-child>
            <Button
              @click="onUnpublish"
              :disabled="
                metadata?.workflow?.pending &&
                !(metadata?.workflow?.stateValid)
              "
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
                  '/metadata/edit/' + metadata.id +
                    '?document=true',
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
      <ContentMetadataEditor
        :documentCollection="documentCollection"
        :parents="parentCollections"
        :relationships="relationships"
        :document="document"
        :template="documentTemplate"
        v-model:metadata="metadata"
        v-model:has-changes="hasChanges"
      />
    </div>
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
