<script setup lang="ts">
import type {CollectionItem} from '~/lib/bosca/contentcollection'
import {computedAsync} from '@vueuse/core'
import {type CollectionInput, CollectionType, type FindAttributes, type MetadataFragment} from '~/lib/graphql/graphql'
import TableFooter from '~/components/ui/table/TableFooter.vue'

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
import {toast} from "~/components/ui/toast";

const client = useBoscaClient()
const router = useRouter()

const selectedId = ref('')
const currentPage = ref(1)
const limit = ref(12)
const offset = computed(() => (currentPage.value - 1) * limit.value)

const {data: rootCollection} = client.collections.findAsyncData({
  attributes: [{attributes: [{key: 'editor.type', value: 'Collections'}]}],
  offset: 0,
  limit: 1,
})

const collections = computedAsync<CollectionItem[]>(async () => {
  if (!rootCollection.value) return []
  const items = (await client.collections.list(rootCollection.value[0].id))?.items || []
  if (selectedId.value == '' && items.length > 0) {
    selectedId.value = items[0].id
  }
  return items
}, [])

const categoryIds = computed(() => {
  for (const collection of collections.value || []) {
    if (collection.id === selectedId.value) {
      return collection.categories.map((c) => c.id)
    }
  }
  return []
})

const { data: templates } = client.metadata.findAsyncData({
  attributes: [],
  contentTypes: ['bosca/v-collection-template'],
  categoryIds: categoryIds,
  offset: 0,
  limit: 1,
})

const findAttributes = computed(() => {
  const editorType = collections.value.find((c) => c.id === selectedId.value)?.attributes['editor.type']
  return [{ attributes: [
      { key: 'editor.type', value: editorType }
    ] } as FindAttributes]
})

const { data: items } = client.collections.findAsyncData({
  attributes: findAttributes,
  categoryIds: categoryIds,
  type: CollectionType.Standard,
  offset: offset,
  limit: limit,
})

const {data: itemsCount} = client.collections.findCountAsyncData({
  attributes: findAttributes,
  categoryIds: categoryIds,
  type: CollectionType.Standard,
})

async function onAdd() {
  for (const collection of collections.value || []) {
    if (collection.id === selectedId.value) {
      const attrs: { [key: string]: string } = {}
      attrs['editor.type'] = collection.attributes['editor.type']
      const template = templates.value ? templates.value[0] : null
      if (!template) {
        toast({
          title: 'No template found',
          description: 'Please create a template first'
        })
        return
      }
      const version = (template as MetadataFragment).version
      const collectionTemplate = await client.metadata.getCollectionTemplate(
          template.id,
          version,
      )
      if (collectionTemplate.defaultAttributes) {
        for (const key in collectionTemplate.defaultAttributes) {
          attrs[key] = collectionTemplate.defaultAttributes[key]
        }
      }
      const newCollection: CollectionInput = {
        parentCollectionId: collection.id,
        name: 'New Collection',
        collectionType: CollectionType.Standard,
        attributes: attrs,
        templateMetadataId: template.id,
        templateMetadataVersion: version,
        categoryIds: collection.categories.map((c) => c.id),
      }
      const collectionId = await client.collections.add(newCollection)
      await client.collections.setReady(collectionId)
      await router.push(`/collections/${collectionId}`)
      break
    }
  }
}

const breadcrumbs = useBreadcrumbs()

onMounted(() => {
  breadcrumbs.set([{title: 'Content'}])
})

watch(selectedId, () => {
  currentPage.value = 1
})
</script>

<template>
  <Tabs v-model:model-value="selectedId" class="h-full space-y-6">
    <div class="flex">
      <TabsList>
        <TabsTrigger v-for="collection in collections" :value="collection.id">
          {{ collection.name }}
        </TabsTrigger>
      </TabsList>
      <div class="grow"></div>
      <div class="flex items-center mr-4">
        <Pagination
            v-slot="{ page }"
            v-model:page="currentPage"
            :total="itemsCount || 0"
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
      <div class="flex items-center">
        <Button @click="onAdd">
          <Icon name="i-lucide-plus"/>
        </Button>
      </div>
    </div>
    <TabsContent
        v-for="collection in collections"
        :value="collection.id"
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
              v-for="item in items"
              :key="item.id"
              @click="router.push(`/collections/${item.id}`)"
              class="cursor-pointer"
          >
            <TableCell class="font-medium flex content-center">
              <NuxtLink :to="'/collections/' + item.id">
                <ContentEditorCollectionItem :collection="item" />
              </NuxtLink>
            </TableCell>
          </TableRow>
        </TableBody>
        <TableFooter>
          <TableRow v-if="!items || items.length === 0">
            <TableCell class="font-medium flex content-center">
              No content found.
            </TableCell>
          </TableRow>
        </TableFooter>
      </Table>
    </TabsContent>
  </Tabs>
</template>
