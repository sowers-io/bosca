<script setup lang="ts">
import {
  type DocumentFragment,
  type DocumentTemplateFragment,
  type GuideFragment,
  type GuideStepFragment,
  type GuideStepModuleFragment,
  type GuideTemplateFragment,
  type MetadataFragment,
  type MetadataRelationshipFragment,
  type ParentCollectionFragment,
  WorkflowStateType,
} from '~/lib/graphql/graphql.ts'
import { TooltipRoot } from 'radix-vue'

const router = useRouter()
const client = useBoscaClient()

const props = defineProps<{
  metadata: MetadataFragment
  relationships: Array<MetadataRelationshipFragment>
  parents: Array<ParentCollectionFragment>
  guide: GuideFragment
  guideTemplate: GuideTemplateFragment | null | undefined
  document: DocumentFragment | null | undefined
  documentTemplate: DocumentTemplateFragment | null | undefined
  currentStep: GuideStepFragment | null | undefined
  currentModule: GuideStepModuleFragment | null | undefined
}>()

const currentStep = defineModel('currentStep', { type: Object, default: null })
const currentModule = defineModel('currentModule', {
  type: Object,
  default: null,
})

const metadata = defineModel<MetadataFragment>('metadata', {
  type: Object,
  default: null,
})

const guideCollection = computed(() => {
  return props.parents?.find((c) => c.attributes['editor.type'] === 'Guide') as
    | ParentCollectionFragment
    | undefined
})
const parentCollections = computed(() => {
  return props.parents?.filter((c) =>
    c.attributes['editor.type'] !== 'Guide'
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
    if (typeof props.metadata.attributes['published'] === 'number') {
      stateValid = new Date(props.metadata.attributes['published'])
    } else {
      stateValid = new Date(Date.parse(props.metadata.attributes['published']))
    }
  }
  if (props.metadata.workflow.state !== published) {
    await client.metadata.beginTransition(
      props.metadata.id,
      props.metadata.version,
      published,
      'Publishing Guide',
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
        'Publishing Guide',
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
      'Unpublishing Guide',
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
  for (const step of props.guide.steps || []) {
    for (const module of step.modules || []) {
      if (module.metadata) {
        await client.metadata.delete(module.metadata.id)
      }
    }
    if (step.metadata) {
      await client.metadata.delete(step.metadata.id)
    }
  }
  await client.metadata.delete(props.metadata.id)
  await router.push('/content')
}

function onBackClick() {
  if (currentModule.value) {
    currentModule.value = null
  } else {
    currentStep.value = null
  }
}

const selected = ref('guide')
const currentPage = ref(1)

watch(currentStep, (step) => {
  selected.value = step ? 'guide' : 'steps'
})
watch(currentModule, (module) => {
  selected.value = module ? 'guide' : 'modules'
})

watch(currentPage, (page) => {
  if (page === 1) {
    currentStep.value = null
    currentModule.value = null
  } else if (page > 1) {
    currentStep.value = props.guide.steps[page - 2]
  }
})
</script>
<template>
  <div>
    <div class="flex items-center">
      <div class="flex">
        <div>
          <Badge variant="secondary">{{ stateName }}
            <span v-if="metadata.workflow.pending">*</span></Badge>
          <Badge variant="secondary" class="ms-4" v-if="pendingStateName">{{
              pendingStateName
            }}
          </Badge>
          <Badge
            variant="outline"
            :class="
              'ms-2 text-gray-400 ' +
              (!hasChanges ? ' invisible' : '')
            "
          >Has Changes
          </Badge>
          <Badge
            variant="destructive"
            :class="'ms-2 ' + (!outOfDate ? ' invisible' : '')"
          >Out of Date
          </Badge>
        </div>
      </div>
      <div class="grow" />
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
            <p>Reset Guide</p>
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
            <p>Save Guide</p>
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
            <p>Preview Guide</p>
          </TooltipContent>
        </Tooltip>
        <Tooltip>
          <TooltipTrigger as-child>
            <Button @click="onDelete" class="flex gap-2" variant="secondary">
              <Icon name="i-lucide-trash" class="size-4" />
            </Button>
          </TooltipTrigger>
          <TooltipContent>
            <p>Delete Guide</p>
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
            <p>Publish Guide</p>
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
            <p>Unpublish Guide</p>
          </TooltipContent>
        </Tooltip>
        <Tooltip>
          <TooltipTrigger as-child>
            <Button
              :disabled="hasChanges"
              @click="
                router.push(
                  '/metadata/edit/' + metadata.id +
                    '?guide=true',
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
      <div class="flex gap-4">
        <TooltipRoot>
          <Pagination
            v-slot="{ page }"
            :items-per-page="1"
            :total="(guide.steps?.length || 0) + 1"
            :sibling-count="6"
            show-edges
            :default-page="1"
            v-model:page="currentPage"
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
                  <Tooltip>
                    <TooltipTrigger as-child>
                      <Button
                        class="h-9"
                        :variant="
                          item.value === page
                          ? 'default'
                          : 'outline'
                        "
                        @click="currentPage = item.value"
                      >
                        <template v-if="item.value === 1">
                          <Icon name="i-lucide-info" class="size-4" />
                        </template>
                        <template v-else>
                          {{ item.value - 1 }}
                        </template>
                      </Button>
                    </TooltipTrigger>
                    <TooltipContent>
                      <template v-if="item.value === 1">
                        Introduction
                      </template>
                      <template v-else>
                        {{
                          guide.steps[item.value - 2]
                          ?.metadata?.name ||
                          'Step ' +
                            (item.value - 2)
                        }}
                      </template>
                    </TooltipContent>
                  </Tooltip>
                </PaginationListItem>
                <PaginationEllipsis v-else :key="item.type" :index="index" />
              </template>
              <PaginationNext />
              <PaginationLast />
            </PaginationList>
          </Pagination>
        </TooltipRoot>
      </div>
      <div class="mt-5" v-if="document">
        <ContentMetadataEditor
          :key="metadata.id"
          :documentCollection="guideCollection"
          :parents="parentCollections"
          :relationships="relationships"
          :document="document"
          :template="documentTemplate"
          v-model:metadata="metadata"
          v-model:has-changes="hasChanges"
        />
      </div>
    </div>
    <Dialog v-model:open="confirmDelete">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Delete Guide</DialogTitle>
          <DialogDescription>
            Are you sure you want to delete this guide?<br />
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <div class="flex w-full items-center gap-4">
            <div class="flex text-sm text-gray-400">This cannot be undone.</div>
            <div class="grow"></div>
            <Button type="button" variant="destructive" @click="doDelete">
              Delete Guide
            </Button>
          </div>
        </DialogFooter>
      </DialogContent>
    </Dialog>
    <Dialog v-model:open="confirmReset">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Reset Guide</DialogTitle>
          <DialogDescription>
            Are you sure you want to reset this guide?<br />
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <div class="flex w-full items-center gap-4">
            <div class="flex text-sm text-gray-400">This cannot be undone.</div>
            <div class="grow"></div>
            <Button type="button" variant="destructive" @click="doReset">
              Reset Guide
            </Button>
          </div>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>
