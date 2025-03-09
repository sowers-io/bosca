<script setup lang="ts">
import { toast } from '~/components/ui/toast'
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
} from '~/lib/graphql/graphql.ts'
import type { BreadcrumbLink } from '~/composables/useBreadcrumbs.ts'

const breadcrumbs = useBreadcrumbs()
const route = useRoute()
const client = useBoscaClient()
const metadata = ref<MetadataFragment>()
const relationships = ref<Array<MetadataRelationshipFragment>>()
const parents = ref<Array<ParentCollectionFragment> | null>()

async function loadMetadata(id: string) {
  metadata.value = await client.metadata.get(id)
  relationships.value = await client.metadata.getRelationships(id)
  parents.value = await client.metadata.getParents(id)
  console.log(id, metadata.value)
}

async function loadDocument(id: string) {
  document.value = await client.metadata.getDocument(id)
  documentTemplate.value =
    document.value.template?.id && document.value.template?.version
      ? await client.metadata.getDocumentTemplate(
        document.value.template?.id,
        document.value.template?.version,
      )
      : null
}

async function loadGuide(id: string) {
  guide.value = await client.metadata.getGuide(
    route.params.metadataId.toString(),
  )
  guideTemplate.value =
    guide.value.template?.id && guide.value.template?.version
      ? await client.metadata.getGuideTemplate(
        guide.value.template?.id,
        guide.value.template?.version,
      )
      : null
  await loadDocument(id)
}

await loadMetadata(route.params.metadataId.toString())

const document = ref<DocumentFragment>()
const documentTemplate = ref<DocumentTemplateFragment | null>()
const guide = ref<GuideFragment>()
const guideTemplate = ref<GuideTemplateFragment | null>()
const currentStep = ref<GuideStepFragment | null>(null)
const currentModule = ref<GuideStepModuleFragment | null>(null)

if (metadata.value?.content.type === 'bosca/v-document') {
  await loadDocument(route.params.metadataId.toString())
} else if (metadata.value?.content.type === 'bosca/v-guide') {
  await loadGuide(route.params.metadataId.toString())
}

watch(currentStep, async (step) => {
  console.log('step', step)
  if (!step) {
    await loadDocument(route.params.metadataId.toString())
    await loadMetadata(route.params.metadataId.toString())
  } else if (step.metadata) {
    await loadDocument(step.metadata.id)
    await loadMetadata(step.metadata.id)
  }
})

watch(currentModule, async (module) => {
  console.log('module', module)
  if (!module && currentStep.value) {
    await loadDocument(currentStep.value.metadata!.id)
    await loadMetadata(currentStep.value.metadata!.id)
  } else if (module?.metadata?.id) {
    await loadDocument(module.metadata.id)
    await loadMetadata(module.metadata.id)
  }
})

client.listeners.onMetadataChanged(async (id) => {
  if (id === metadata.value?.id) {
    try {
      document.value = await client.metadata.getDocument(id)
      parents.value = await client.metadata.getParents(id)
      relationships.value = await client.metadata.getRelationships(id)
      metadata.value = await client.metadata.get(id)
      if (guide.value) {
        guide.value = await client.metadata.getGuide(
          route.params.metadataId.toString(),
        )
      }
      toast({ title: 'Content Updated.' })
    } catch (ignore) {
    }
  }
})

onMounted(() => {
  const items: BreadcrumbLink[] = [
    { title: 'Content', to: '/content' },
  ]
  if (guide.value) {
    items.push({ title: 'Edit Guide' })
  } else {
    items.push({ title: 'Edit Document' })
  }
  breadcrumbs.set(items)
})
</script>
<template>
  <ContentMetadataGuideEditor
    v-if="metadata && guide && document"
    v-model:metadata="metadata"
    :guide="guide"
    :guideTemplate="guideTemplate"
    v-model:document="document"
    v-model:documentTemplate="documentTemplate"
    :parents="parents || []"
    :relationships="relationships || []"
    v-model:currentStep="currentStep"
    v-model:currentModule="currentModule"
  />
  <ContentMetadataDocumentEditor
    v-else-if="metadata && document"
    :metadata="metadata"
    :document="document"
    :documentTemplate="documentTemplate"
    :parents="parents || []"
    :relationships="relationships || []"
  />
</template>
