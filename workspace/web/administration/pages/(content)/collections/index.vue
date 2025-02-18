<script setup lang="ts">
import { toast } from '~/components/ui/toast'

const router = useRouter()
const route = useRoute()
const collectionId = computed(() =>
  route.query.id?.toString() || '00000000-0000-0000-0000-000000000000'
)
const dropZoneRef = ref<HTMLDivElement>()
const client = useBoscaClient()
const { data: list, refresh } = client.collections.listAsyncData(collectionId)
const { data: parent } = client.collections.getCollectionParentAsyncData(
  collectionId,
)

async function onDrop(files: File[] | null) {
  if (!files) return
  toast({
    title: 'Uploading files, please wait...',
  })
  try {
    await client.metadata.addFiles(collectionId.value, files)
    toast({
      title: 'File(s) uploaded',
    })
  } catch (e) {
    console.error('Error uploading file(s)', e)
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
  if (id == list.value?.collection?.id) {
    refresh()
  }
})

client.listeners.onMetadataChanged((id) => {
  for (const item of list.value?.items || []) {
    if (item.id === id) {
      refresh()
    }
  }
})

const breadcrumbs = useBreadcrumbs()

function onUpdateBreadcrumbs() {
  let links: BreadcrumbLink[] = [{ title: 'Collections', to: '/collections' }]
  if (parent.value) {
    if (parent.value.id !== '00000000-0000-0000-0000-000000000000') {
      links.push({ title: '...' })
    }
    links.push({
      title: parent.value.name,
      to: '/collections?id=' + parent.value.id,
    })
  }
  if (list.value?.collection) {
    links.push({ title: list.value.collection.name })
  }
  breadcrumbs.set(links)
}

watch(list, onUpdateBreadcrumbs)
watch(router.currentRoute, onUpdateBreadcrumbs)

onMounted(onUpdateBreadcrumbs)
</script>

<template>
  <div ref="dropZoneRef" class="w-full h-full">
    <div class="flex items-center gap-4">
      <Button
        variant="ghost"
        @click="
          router.push(
            '/collections?id=' +
              (parent?.id || '00000000-0000-0000-0000-000000000000'),
          )
        "
      >
        <Icon
          name="i-lucide-arrow-left"
          :class="'size-4' + (parent ? '' : ' opacity-10')"
          type="svg"
        />
      </Button>
      {{ list?.collection?.name }}
      <div class="grow"></div>
      <ContentCollectionsAddSheet
        :parent="list?.collection?.id"
        v-if="list?.collection?.id"
      />
    </div>
    <ContentCollectionsList
      v-if="list"
      :collection="list.collection"
      :items="list.items"
    />
  </div>
</template>
