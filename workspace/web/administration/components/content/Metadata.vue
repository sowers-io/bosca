<script lang="ts" setup>
const props = defineProps<{
  metadataId: string
}>()

const client = useBoscaClient()
const { data: metadata, refresh } = client.metadata.getAsyncData(
  props.metadataId,
)
const router = useRouter()
const confirmDelete = ref(false)

function onDelete() {
  confirmDelete.value = true
}

function doDelete() {
  client.metadata.delete(metadata.value!.id)
  router.back()
  confirmDelete.value = false
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
  <div>
    <Tabs default-value="general" class="h-full space-y-6" v-if="metadata">
      <div class="flex">
        <TabsList>
          <TabsTrigger value="general">
            General
          </TabsTrigger>
          <TabsTrigger value="traits">
            Traits
          </TabsTrigger>
          <TabsTrigger value="attributes">
            Attributes
          </TabsTrigger>
          <TabsTrigger
            value="content"
            v-if="
              metadata.uploaded ||
              metadata.content.type === 'bosca/v-document'
            "
          >
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
      <TabsContent value="attributes" class="border-none p-0 outline-none">
        <ContentAttributes class="col-span-2" :content="metadata" />
      </TabsContent>
      <TabsContent value="supplementary" class="border-none p-0 outline-none">
        <ContentMetadataSupplementary class="col-span-2" :metadata="metadata" />
      </TabsContent>
      <TabsContent value="workflows" class="border-none p-0 outline-none">
        <ContentWorkflows class="col-span-2" :content="metadata" />
      </TabsContent>
    </Tabs>
    <Dialog v-model:open="confirmDelete">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Delete Metadata</DialogTitle>
          <DialogDescription>
            Are you sure you want to delete this metadata? It will also delete
            any documents or supplementary data.<br />
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <div class="flex w-full items-center gap-4">
            <div class="flex text-sm text-gray-400">This cannot be undone.</div>
            <div class="grow"></div>
            <Button type="button" variant="destructive" @click="doDelete">
              Delete Metadata
            </Button>
          </div>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>
