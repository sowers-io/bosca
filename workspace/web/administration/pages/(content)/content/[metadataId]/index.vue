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

const document = ref<DocumentFragment>()
const documentTemplate = ref<DocumentTemplateFragment | null>()
const guide = ref<GuideFragment>()
const guideTemplate = ref<GuideTemplateFragment | null>()
const currentStep = ref<GuideStepFragment | null>(null)
const currentModule = ref<GuideStepModuleFragment | null>(null)

async function loadMetadata(id: string) {
  metadata.value = await client.metadata.get(id)
  relationships.value = await client.metadata.getRelationships(id)
  parents.value = await client.metadata.getParents(id)
}

async function loadDocument(id: string) {
  const d = await client.metadata.getDocument(id)
  if (d) {
    documentTemplate.value = d.template?.id && d.template?.version
      ? await client.metadata.getDocumentTemplate(
        d.template?.id,
        d.template?.version,
      )
      : null
    document.value = d
  } else {
    document.value = {
      title: 'Empty Document',
      content: [],
    }
  }
  await loadMetadata(id)
}

async function loadGuide(id: string) {
  const g = await client.metadata.getGuide(id)
  guideTemplate.value = g.template?.id && g.template?.version
    ? await client.metadata.getGuideTemplate(
      g.template?.id,
      g.template?.version,
    )
    : null
  guide.value = g
  await loadDocument(id)
}

watch(currentStep, async (step) => {
  if (!step) {
    await loadDocument(route.params.metadataId.toString())
    await loadMetadata(route.params.metadataId.toString())
  } else if (step.metadata) {
    console.log(step.metadata.id, step.metadata.name, step.metadata.type)
    await loadDocument(step.metadata.id)
    await loadMetadata(step.metadata.id)
  }
})

watch(currentModule, async (module) => {
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

onMounted(async () => {
  await loadMetadata(route.params.metadataId.toString())
  if (metadata.value?.content.type === 'bosca/v-document') {
    await loadDocument(route.params.metadataId.toString())
  } else if (metadata.value?.content.type === 'bosca/v-guide') {
    await loadGuide(route.params.metadataId.toString())
  }
  const items: BreadcrumbLink[] = [
    { title: 'Content', to: '/content' },
  ]
  items.push({ title: 'Manage ' + (metadata.value?.attributes['type']) })
  if (metadata.value?.name) {
    items.push({ title: metadata.value?.name })
  }
  breadcrumbs.set(items)
})
</script>
<template>
  <ContentMetadataGuideEditor
    v-if="metadata && guide && document"
    :guide="guide"
    :guideTemplate="guideTemplate"
    :parents="parents || []"
    :relationships="relationships || []"
    v-model:metadata="metadata"
    v-model:document="document"
    v-model:documentTemplate="documentTemplate"
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
