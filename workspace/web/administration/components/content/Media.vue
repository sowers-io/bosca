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
import type { Reactive } from 'vue'
import type { MetadataInput } from '~/lib/graphql/graphql.ts'

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

const youtube = ref('')
async function onAddYouTube() {
  console.log('onAdd', youtube.value)
  if (!youtube.value) return
  const metadata = {
    name: 'YouTube ' + youtube.value,
    contentType: 'bosca/x-youtube-video',
    languageTag: 'en',
    attributes: {
      'youtube.id': youtube.value,
    },
  } as MetadataInput
  const id = await client.metadata.add(metadata)
  if (props.onSelected) {
    props.onSelected(id)
  }
}

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
    const ids = await uploader.upload(files)
    toast({ title: 'Media File(s) uploaded' })
    if (props.onSelected) {
      for (const id of ids) {
        props.onSelected(id)
        break
      }
    }
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
  <div class="h-full w-full relative">
    <div
      class="flex justify-between items-center absolute top-4 left-0 right-0 h-8 px-4"
    >
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
    <div
      ref="dropZoneRef"
      class="absolute right-0 left-0 top-16 bottom-20 overflow-auto"
    >
      <div class="grid grid-cols-6 w-full mt-6 gap-x-6 gap-y-14">
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
    <div
      class="flex justify-items-center absolute bottom-0 w-full h-14 border rounded-md p-2"
    >
      <div class="grow"></div>
      <div class="grid place-content-center h-full text-gray-400 mr-2">
        Drag and drop files above or provide a YouTube ID
      </div>
      <div class="flex gap-2">
        <Input placeholder="Add YouTube ID" v-model="youtube" />
        <Button @click="onAddYouTube">
          <Icon name="i-lucide-plus" />
        </Button>
      </div>
    </div>
  </div>
</template>
