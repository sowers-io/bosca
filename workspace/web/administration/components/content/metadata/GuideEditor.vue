<script setup lang="ts">
import {
  type DocumentFragment,
  type DocumentTemplateFragment,
  type GuideFragment,
  type GuideStepFragment,
  type GuideStepModuleFragment,
  type GuideTemplateFragment,
  GuideType,
  type MetadataFragment,
  type MetadataRelationshipFragment,
  type ParentCollectionFragment,
  WorkflowStateType,
} from '~/lib/graphql/graphql.ts'
import { type DateValue, getLocalTimeZone } from '@internationalized/date'

const router = useRouter()
const client = useBoscaClient()

const props = defineProps<{
  guideMetadataId: string
  guideMetadataVersion: number
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

const loading = ref(false)
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
  startLoading()
  const states = await client.workflows.getStates() || []
  const published =
    states.find((s) => s.type === WorkflowStateType.Published)?.id || ''
  let stateValid: Date | null = null
  if (props.metadata.attributes && props.metadata.attributes['published']) {
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
  await client.metadata.deleteGuide(
    props.guideMetadataId,
    props.guideMetadataVersion,
  )
  await router.push('/content')
}

const selected = ref('guide')
const currentPage = ref(1)
const modules = ref([])

const currentDate = ref<DateValue>()

watch(currentDate, () => {
  const date = currentDate.value?.toDate(getLocalTimeZone())?.getTime() || 0
  let index = 1
  for (const step of props.guide.steps) {
    const d = step.date ? Date.parse(step.date) : 0
    if (d >= date) {
      break
    }
    index++
  }
  currentPage.value = index
})

async function buildModules() {
  const modulesParts = []
  if (props.document) {
    modulesParts.push({
      metadata: props.metadata,
      document: props.document,
      documentTemplate: props.documentTemplate,
    })
  }
  for (const module of currentStep.value?.modules || []) {
    const metadata = module.metadata
    const document = await client.metadata.getDocument(metadata.id)
    const documentTemplate = document.template
      ? await client.metadata.getDocumentTemplate(
        document.template.id,
        document.template.version,
      )
      : null
    modulesParts.push({
      metadata: metadata,
      document: document,
      documentTemplate: documentTemplate,
    })
  }
  // @ts-ignore
  modules.value = modulesParts
}

watch(currentStep, (step) => {
  selected.value = step ? 'guide' : 'steps'
  buildModules()
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

onUpdated(() => {
  if (currentPage.value === 1) {
    currentStep.value = null
    currentModule.value = null
  } else if (currentPage.value > 1) {
    currentStep.value = props.guide.steps[currentPage.value - 2]
  }
})

let loadingIntervalCheck: any

function startLoading() {
  loading.value = true
  if (loadingIntervalCheck) clearInterval(loadingIntervalCheck)
  const id = props.metadata.id
  loadingIntervalCheck = setInterval(async () => {
    if (!loading.value) {
      clearInterval(loadingIntervalCheck)
      return
    }
    const running = await client.metadata.getRunningWorkflowCount(id)
    if (!running) {
      loading.value = false
      clearInterval(loadingIntervalCheck)
    }
  }, 1000)
}

function formatDate(date: string) {
  const d = new Date(Date.parse(date))
  return new Date(d.getUTCFullYear(), d.getUTCMonth(), d.getUTCDate())
    .toLocaleDateString('en', {
      year: '2-digit',
      month: 'numeric',
      day: 'numeric',
    })
}

onMounted(() => {
  buildModules()
})

async function onAddStep() {
  if (!props.guideTemplate) return
  startLoading()
  const templateStep = props.guideTemplate.steps[0]
  if (!templateStep.metadata) return
  const id = await client.metadata.addGuideStep(
    props.guideMetadataId,
    props.guideMetadataVersion,
    currentPage.value - 1,
    templateStep.metadata.id,
    templateStep.metadata.version,
    templateStep.id,
  )
  await client.metadata.setReady(id)
  currentPage.value = currentPage.value + 1
}

async function onDeleteStep() {
  const id = currentStep.value?.id
  if (!id) return
  startLoading()
  await client.metadata.deleteGuideStep(
    props.guideMetadataId,
    props.guideMetadataVersion,
    id,
  )
  currentPage.value = currentPage.value - 1
}
</script>
<template>
  <div>
    <div class="flex items-center">
      <div class="flex">
        <div>
          <Badge variant="secondary">{{ stateName }}<span
              v-if="metadata.workflow.pending"
            >*</span></Badge>
          <Badge variant="secondary" class="ms-4" v-if="pendingStateName">{{
            pendingStateName
          }}</Badge>
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
          <Badge
            variant="outline"
            :class="
              'ms-2 ' + (loading || metadata.workflow.running > 0
              ? ''
              : ' invisible')
            "
          >
            <Icon
              name="i-lucide-loader-circle"
              class="size-4 text-primary animate-spin mr-2"
            />
            Workflow Running
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
      <div class="flex gap-6 w-full">
        <Popover
          v-if="
            guide.type === GuideType.Calendar ||
            guide.type === GuideType.CalendarProgress
          "
        >
          <PopoverTrigger as-child>
            <Button variant="outline">
              <Icon name="i-lucide-calendar" class="h-4 w-4" />
            </Button>
          </PopoverTrigger>
          <PopoverContent class="w-auto p-0">
            <Calendar v-model="currentDate" initial-focus />
          </PopoverContent>
        </Popover>
        <Pagination
          v-slot="{ page }"
          :items-per-page="1"
          :total="(guide.steps?.length || 0) + 1"
          :sibling-count="guide.type === GuideType.Calendar ? 2 : 6"
          show-edges
          :default-page="1"
          v-model:page="currentPage"
        >
          <PaginationList v-slot="{ items }" class="flex items-center gap-1">
            <PaginationFirst :disabled="hasChanges" />
            <PaginationPrev :disabled="hasChanges" />
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
                      :disabled="hasChanges"
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
                        <template
                          v-if="
                            guide.type ===
                            GuideType.Calendar
                          "
                        >
                          {{
                            formatDate(
                              guide
                                .steps[
                                  item.value - 2
                                ].date,
                            )
                          }}
                        </template>
                        <template v-else>
                          {{ item.value - 1 }}
                        </template>
                      </template>
                    </Button>
                  </TooltipTrigger>
                  <TooltipContent>
                    <template v-if="item.value === 1">
                      Introduction
                    </template>
                    <template v-else>
                      {{
                        guide.type ===
                          GuideType.Calendar
                        ? formatDate(
                          guide.steps[item.value - 2]
                            .date,
                        )
                        : guide.steps[item.value - 2]
                          ?.metadata?.name ||
                          'Step ' + (item.value - 2)
                      }}
                    </template>
                  </TooltipContent>
                </Tooltip>
              </PaginationListItem>
              <PaginationEllipsis v-else :key="item.type" :index="index" />
            </template>
            <PaginationNext :disabled="hasChanges" />
            <PaginationLast :disabled="hasChanges" />
          </PaginationList>
        </Pagination>
        <div class="flex gap-2">
          <Button @click="onAddStep" class="flex gap-2" :disabled="hasChanges">
            <Icon name="i-lucide-plus" class="size-4" />
          </Button>
          <Button
            @click="onDeleteStep"
            class="flex"
            variant="ghost"
            :disabled="hasChanges"
            v-if="currentStep && metadata.workflow.state !== 'published'"
          >
            <Icon name="i-lucide-trash" class="size-4" />
          </Button>
        </div>
      </div>
      <div class="flex mt-5 justify-center" v-if="document">
        <Carousel
          class="relative w-[calc(100dvw-400px)] h-[calc(100dvh-170px)]"
          v-if="modules && modules.length > 1"
        >
          <CarouselContent>
            <CarouselItem v-for="(module, index) in modules" :key="index">
              <ContentMetadataEditor
                :key="module.metadata.id"
                :documentCollection="guideCollection"
                :parents="parentCollections"
                :relationships="relationships"
                :document="module.document"
                :template="module.documentTemplate"
                v-model:metadata="module.metadata"
                v-model:has-changes="hasChanges"
              />
            </CarouselItem>
          </CarouselContent>
          <CarouselPrevious />
          <CarouselNext />
        </Carousel>
        <ContentMetadataEditor
          v-else
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
