<script setup lang="ts">
import type { ContentTypeFilter } from '~/lib/bosca/contentmetadata'
import { toast } from '~/components/ui/toast'
import { Uploader } from '~/lib/uploader'
import {
  Pagination,
  PaginationEllipsis,
  PaginationFirst,
  PaginationLast,
  PaginationList,
  PaginationListItem,
  PaginationNext,
  PaginationPrev,
} from '@/components/ui/pagination'
import type {Reactive} from "vue";

const props = defineProps<{
  onSelected?: (id: string) => void
  filter: Reactive<ContentTypeFilter>
}>()

const dropZoneRef = ref<HTMLDivElement>()
const currentPage = ref(1)
const limit = ref(18)
const offset = computed(() => (currentPage.value - 1) * limit.value)

const client = useBoscaClient()
const uploader = new Uploader(client)

const { data: metadata, refresh: refreshList } = client.metadata
  .getByContentType(
    props.filter,
    offset,
    limit,
  )

const { data: count, refresh: refreshCount } = client.metadata
  .getByContentTypeCount(
      props.filter,
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
    refreshList()
    refreshCount()
  }
})

client.listeners.onMetadataChanged((id) => {
  const item = metadata.value?.find((m) => m.id == id)
  if (item) {
    refreshList()
    refreshCount()
  }
})
</script>

<template>
  <div ref="dropZoneRef" class="h-full">
    <div class="flex justify-between items-center">
      <ContentMultiContentTypeFilter :filter="filter" />
      <Pagination
        v-slot="{ page }"
        v-model:page="currentPage"
        :total="count || 0"
        :items-per-page="limit"
        :sibling-count="1"
        show-edges
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
              <Button
                class="w-10 h-10 p-0"
                :variant="item.value === page ? 'default' : 'outline'"
              >
                {{ item.value }}
              </Button>
            </PaginationListItem>
            <PaginationEllipsis v-else :key="item.type" :index="index" />
          </template>

          <PaginationNext />
          <PaginationLast />
        </PaginationList>
      </Pagination>
    </div>
    <div class="grid grid-cols-6 w-full mt-6 gap-x-6 gap-y-14 pb-8">
      <ContentMetadataImage
        v-for="item in metadata"
        :key="item.id"
        :metadata="item"
        class="w-[200px] h-[300px]"
        aspect-ratio="portrait"
        :on-selected="onSelected"
      />
    </div>
  </div>
</template>
