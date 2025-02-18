<script setup lang="ts">
import type { ContentTypeFilter } from '~/lib/bosca/contentmetadata'
import { toast } from '~/components/ui/toast'
import { Uploader } from '~/lib/uploader'

const props = defineProps<{
  onSelected?: (id: string) => void
  filter: ContentTypeFilter
}>()

const filter = ref<ContentTypeFilter>(props.filter)
const dropZoneRef = ref<HTMLDivElement>()
const offset = ref(0)
const limit = ref(20)

const client = useBoscaClient()
const uploader = new Uploader(client)

const { data: metadata, refresh } = client.metadata.getByContentType(
  filter,
  offset,
  limit,
)

async function onDrop(files: File[] | null) {
  if (!files) return
  toast({ title: 'Uploading media files, please wait...' })
  try {
    await uploader.upload(files)
    toast({ title: 'Media File(s) uploaded' })
  } catch (e) {
    toast({
      title: 'Error uploading file(s)',
      description: (e as unknown as any).message,
    })
  }
}

useDropZone(dropZoneRef, {
  onDrop,
  multiple: true,
  preventDefaultForUnhandled: false,
})

client.listeners.onCollectionChanged((id) => {
  if (uploader.isAssetCollection(id)) {
    refresh()
  }
})

client.listeners.onMetadataChanged((id) => {
  const item = metadata.value?.find((m) => m.id == id)
  if (item) {
    refresh()
  }
})
</script>

<template>
  <div ref="dropZoneRef" class="h-full">
    <ContentMultiContentTypeFilter v-model:model-value="filter" />
    <div class="flex flex-wrap mt-6 gap-4 pb-4">
      <ContentMetadataImage
        v-for="item in metadata"
        :key="item.id"
        :metadata="item"
        class="w-[250px]"
        aspect-ratio="portrait"
        :width="250"
        :height="330"
        :on-selected="onSelected"
      />
    </div>
  </div>
</template>
