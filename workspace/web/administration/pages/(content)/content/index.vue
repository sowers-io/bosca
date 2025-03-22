<script setup lang="ts">
import type { CollectionItem } from '~/lib/bosca/contentcollection'
import type { MetadataFragment } from '~/lib/graphql/graphql'
import { computedAsync } from '@vueuse/core'
import TableFooter from '~/components/ui/table/TableFooter.vue'
import { toast } from '~/components/ui/toast'
import {
  Pagination,
  PaginationEllipsis,
  PaginationFirst,
  PaginationLast,
  PaginationList,
  PaginationListItem,
  PaginationNext,
  PaginationPrev,
} from '~/components/ui/pagination'
import { useStorage } from '@vueuse/core'

const client = useBoscaClient()
const router = useRouter()

const currentContentPage = useStorage('selected-content-page', 1)
const selectedContentTab = useStorage('selected-content-tab', '')
const selectedId = ref('')
const currentPage = ref(1)
const limit = ref(12)
const offset = computed(() => (currentPage.value - 1) * limit.value)

watch(selectedId, () => {
  if (selectedContentTab.value != selectedId.value) {
    selectedContentTab.value = selectedId.value
    currentPage.value = 1
  }
})

watch(currentPage, () => {
  currentContentPage.value = currentPage.value
})

const { data: collection } = client.collections.findAsyncData({
  attributes: [{
    attributes: [{ key: 'editor.type', value: 'DocumentsAndGuides' }],
  }],
  offset: 0,
  limit: 1,
})

const collectionItems = computedAsync<CollectionItem[]>(async () => {
  if (!collection.value) return []
  const items = (await client.collections.list(collection.value[0].id))?.items || []
  if (selectedId.value == '' && items.length > 0) {
    selectedId.value = items[0].id
  }
  return items
}, [])

const categoryIds = computed(() => {
  for (const collection of collectionItems.value || []) {
    if (collection.id === selectedId.value) {
      return collection.categories.map((c) => c.id)
    }
  }
  return []
})

const contentTypes = computed(() => {
  for (const collection of collectionItems.value || []) {
    if (collection.id === selectedId.value) {
      return ['bosca/v-' + collection.attributes['editor.type'].toLowerCase()]
    }
  }
  return []
})

const { data: items } = client.metadata.findAsyncData({
  attributes: [],
  contentTypes: contentTypes,
  categoryIds: categoryIds,
  offset: offset,
  limit: limit,
})

const { data: count } = client.metadata.findCountAsyncData({
  attributes: [],
  contentTypes: contentTypes,
  categoryIds: categoryIds,
  offset: offset,
  limit: limit,
})

const content = computed(() => {
  return items.value?.filter((i) =>
    i.attributes && !i.attributes['template.type']
  ) || []
})

async function onAddDocument(
  parentCollectionId: string,
  template: MetadataFragment,
) {
  const id = await client.metadata.addDocument(
    parentCollectionId,
    template.id,
    template.version,
  )
  await client.metadata.setReady(id)
  await router.push(`/content/${id}`)
}

async function onAddGuide(
  parentCollectionId: string,
  template: MetadataFragment,
) {
  const id = await client.metadata.addGuide(
    parentCollectionId,
    template.id,
    template.version,
  )
  await client.metadata.setReady(id)
  await router.push(`/content/${id}`)
}

async function onAdd() {
  for (const item of collectionItems.value || []) {
    if (item.id === selectedId.value) {
      const templates = await client.metadata.find({
        attributes: [],
        contentTypes: ['bosca/v-' + item.attributes['editor.type'].toLowerCase() + '-template'],
        categoryIds: categoryIds,
        offset: 0,
        limit: 1,
      })
      const template = templates[0]
      if (!template) {
        toast({
          title: 'No template found',
          description: 'Please create a template first',
        })
        return
      }
      if (item.attributes['editor.type'] === 'Document') {
        await onAddDocument(item.id, template)
      } else if (item.attributes['editor.type'] === 'Guide') {
        await onAddGuide(item.id, template)
      }
      break
    }
  }
}

const breadcrumbs = useBreadcrumbs()

onMounted(() => {
  breadcrumbs.set([{ title: 'Content' }])
  selectedId.value = selectedContentTab.value || ''
  currentPage.value = currentContentPage.value
})
</script>

<template>
  <Tabs class="h-full space-y-6" v-model:model-value="selectedId">
    <div class="flex">
      <TabsList>
        <TabsTrigger
          v-for="collectionItem in collectionItems"
          :value="collectionItem.id"
        >
          {{ collectionItem.name }}
        </TabsTrigger>
      </TabsList>
      <div class="grow"></div>
      <div class="flex items-center mr-4">
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
                  :variant="
                    item.value === page
                    ? 'default'
                    : 'outline'
                  "
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
      <div class="flex items-center">
        <Button @click="onAdd">
          <Icon name="i-lucide-plus" />
        </Button>
      </div>
    </div>
    <TabsContent
      v-for="collectionItem in collectionItems"
      :value="collectionItem.id"
      class="border-none p-0 mt-0 outline-none"
    >
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Name</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow
            v-for="item in content"
            :key="item.id"
            @click="router.push(`/content/${item.id}`)"
            class="cursor-pointer"
          >
            <TableCell class="font-medium flex content-center">
              <NuxtLink :to="'/content/' + item.id">
                <ContentListItem :item="item" />
              </NuxtLink>
            </TableCell>
          </TableRow>
        </TableBody>
        <TableFooter>
          <TableRow v-if="content.length === 0">
            <TableCell class="font-medium flex content-center">
              No content found.
            </TableCell>
          </TableRow>
        </TableFooter>
      </Table>
    </TabsContent>
  </Tabs>
</template>
