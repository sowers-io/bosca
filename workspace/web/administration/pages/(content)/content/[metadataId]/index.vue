<script setup lang="ts">
import { toast } from '~/components/ui/toast'

const breadcrumbs = useBreadcrumbs()
const route = useRoute()
const selectedItem = ref('document')

const client = useBoscaClient()

let metadata = ref(
  await client.metadata.get(route.params.metadataId.toString()),
)
let document = ref(
  await client.metadata.getDocument(route.params.metadataId.toString()),
)
let template = ref(
  document.value.templateMetadataId && document.value.templateMetadataVersion
    ? await client.metadata.getDocumentTemplate(
      document.value.templateMetadataId,
      document.value.templateMetadataVersion,
    )
    : null,
)

function onSave() {
  window.dispatchEvent(new Event('save-document'))
}

client.listeners.onMetadataChanged(async (id) => {
  if (id === metadata.value.id) {
    metadata.value = await client.metadata.get(id)
    toast({ title: 'Document updated.' })
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
  <Tabs v-model:model-value="selectedItem" class="h-full space-y-6">
    <div class="flex">
      <TabsList>
        <TabsTrigger value="document">
          Document
        </TabsTrigger>
        <TabsTrigger value="metadata">
          Metadata
        </TabsTrigger>
      </TabsList>
      <div class="grow"></div>
      <div
        v-if="
          selectedItem === 'document' &&
          metadata?.workflow?.state === 'draft'
        "
        class="flex gap-2"
      >
        <Button @click="onSave" class="flex gap-2">
          <Icon name="i-lucide-save" class="size-4 me-2" />
          Save
        </Button>
      </div>
    </div>
    <TabsContent
      value="document"
      class="border-none p-0 outline-none"
      force-mount
    >
      <ContentMetadataEditor
        :metadata="metadata"
        :document="document"
        :template="template"
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
</template>
