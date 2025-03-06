<script setup lang="ts">
import { toast } from '~/components/ui/toast'
import {
  type DocumentFragment,
  type DocumentTemplateFragment,
  type GuideFragment,
  type GuideTemplateFragment,
} from '~/lib/graphql/graphql.ts'

const breadcrumbs = useBreadcrumbs()
const route = useRoute()
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
const document = ref<DocumentFragment>()
const documentTemplate = ref<DocumentTemplateFragment | null>()
const guide = ref<GuideFragment>()
const guideTemplate = ref<GuideTemplateFragment | null>()

if (metadata.value.content.type === 'bosca/v-document') {
  document.value = await client.metadata.getDocument(
    route.params.metadataId.toString(),
  )
  documentTemplate.value =
    document.value.template?.id && document.value.template?.version
      ? await client.metadata.getDocumentTemplate(
        document.value.template?.id,
        document.value.template?.version,
      )
      : null
} else {
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
}

client.listeners.onMetadataChanged(async (id) => {
  if (id === metadata.value.id) {
    try {
      document.value = await client.metadata.getDocument(id)
      parents.value = await client.metadata.getParents(id)
      relationships.value = await client.metadata.getRelationships(id)
      metadata.value = await client.metadata.get(id)
      toast({ title: 'Content Updated.' })
    } catch (ignore) {
    }
  }
})

onMounted(() => {
  breadcrumbs.set([
    { title: 'Content', to: '/content' },
    { title: 'Edit ' + (document.value ? 'Document' : 'Guide') },
  ])
})
</script>
<template>
  <ContentMetadataDocumentEditor
    v-if="document"
    :metadata="metadata"
    :document="document"
    :documentTemplate="documentTemplate"
    :parents="parents || []"
    :relationships="relationships"
  />
  <ContentMetadataGuideEditor
    v-if="guide"
    :metadata="metadata"
    :guide="guide"
    :guideTemplate="guideTemplate"
    :parents="parents || []"
    :relationships="relationships"
  />
</template>
