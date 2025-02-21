<script lang="ts" setup>
const props = defineProps<{
  metadataId: string
}>()

const client = useBoscaClient()
const { data: metadata, refresh } = client.metadata.getAsyncData(
  props.metadataId,
)
const router = useRouter()

function onDelete() {
  client.metadata.delete(metadata.value!.id)
  router.back()
}

client.listeners.onMetadataChanged((id) => {
  if (metadata.value?.id === id) {
    refresh()
  }
})

client.listeners.onMetadataSupplementaryChanged((id) => {
  if (metadata.value?.id === id) {
    refresh()
  }
})
</script>
<template>
  <Tabs default-value="general" class="h-full space-y-6" v-if="metadata">
    <div class="flex">
      <TabsList>
        <TabsTrigger value="general">
          General
        </TabsTrigger>
        <TabsTrigger value="traits">
          Traits
        </TabsTrigger>
        <TabsTrigger value="content" v-if="metadata.uploaded">
          Content
        </TabsTrigger>
        <TabsTrigger value="supplementary">
          Supplementary
        </TabsTrigger>
        <TabsTrigger value="workflows">
          Workflows
        </TabsTrigger>
      </TabsList>
      <div class="grow"></div>
      <div>
        <Button variant="outline" @click="onDelete">
          <Icon name="i-lucide-trash" />
        </Button>
      </div>
    </div>
    <TabsContent value="general" class="border-none p-0 outline-none">
      <div class="flex flex-wrap gap-8">
        <ContentMetadataDetails :metadata="metadata" class="w-[600px]" />
        <ContentVisibility :content="metadata" class="w-[600px]" />
        <ContentStates :content="metadata" class="w-[600px]" />
        <ContentContents :content="metadata" class="w-[600px]" />
      </div>
    </TabsContent>
    <TabsContent value="content" class="border-none p-0 outline-none">
      <ContentMetadataContent class="col-span-2" :metadata="metadata" />
    </TabsContent>
    <TabsContent value="traits" class="border-none p-0 outline-none">
      <ContentTraits class="col-span-2" :content="metadata" />
    </TabsContent>
    <TabsContent value="supplementary" class="border-none p-0 outline-none">
      <ContentMetadataSupplementary class="col-span-2" :metadata="metadata" />
    </TabsContent>
    <TabsContent value="workflows" class="border-none p-0 outline-none">
      <ContentWorkflows class="col-span-2" :content="metadata" />
    </TabsContent>
  </Tabs>
</template>
