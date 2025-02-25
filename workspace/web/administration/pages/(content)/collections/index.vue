<script setup lang="ts">
import type {CollectionItem} from '~/lib/bosca/contentcollection'
import {computedAsync} from '@vueuse/core'
import {
  type CollectionInput,
  CollectionType,
  type FindAttribute,
  type FindAttributes,
  type MetadataFragment
} from '~/lib/graphql/graphql'
import TableFooter from '~/components/ui/table/TableFooter.vue'

const client = useBoscaClient()
const router = useRouter()

const selectedId = ref('')
const attributes = ref<FindAttribute[]>([])
const categoryIds = ref<string[]>([])
const offset = ref(0)
const limit = ref(0)

const {data: rootCollection} = client.collections.findAsyncData({
  attributes: [{attributes: [{key: 'editor.type', value: 'Collections'}]}],
  offset: 0,
  limit: 1,
})

const findAttributes = computed(() => {
  return [{ attributes: attributes.value } as FindAttributes]
})

const {data: collectionItems} = client.collections.findAsyncData({
  attributes: findAttributes,
  categoryIds: categoryIds,
  type: CollectionType.Standard,
  offset: offset,
  limit: limit,
})

const rootCollectionItems = computedAsync<CollectionItem[]>(async () => {
  if (!rootCollection.value) return []
  const items = (await client.collections.list(rootCollection.value[0].id))?.items || []
  if (selectedId.value == '' && items.length > 0) {
    selectedId.value = items[0].id
    limit.value = 50
  }
  return items
}, [])

const rootCollections = computed(() => {
  return rootCollectionItems.value?.filter((i) =>
      i.attributes && !i.attributes['template.type']
  ) || []
})

const rootTemplates = computed(() => {
  return rootCollectionItems.value?.filter((i) =>
      i.attributes && i.attributes['template.type']
  ) || []
})

function updateAttributes() {
  for (const collection of rootCollections.value || []) {
    if (collection.id === selectedId.value) {
      attributes.value = [
        { key: 'editor.type', value: collection.attributes['editor.type'] }
      ]
      categoryIds.value = collection.categories.map((c) => c.id)
      break
    }
  }
}
watch(rootCollections, updateAttributes)

async function onAdd() {
  for (const collection of rootCollections.value || []) {
    if (collection.id === selectedId.value) {
      const attrs: { [key: string]: string } = {}
      attrs['editor.type'] = collection.attributes['template.type']
      const template = rootTemplates.value[0]
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
</script>

<template>
  <Tabs
      v-model:model-value="selectedId"
      class="h-full space-y-6"
      @update:model-value="updateAttributes"
  >
    <div class="flex">
      <TabsList>
        <TabsTrigger v-for="collection in rootCollections" :value="collection.id">
          {{ collection.name }}
        </TabsTrigger>
      </TabsList>
      <div class="grow"></div>
      <div>
        <Button @click="onAdd">
          <Icon name="i-lucide-plus"/>
        </Button>
      </div>
    </div>
    <TabsContent
        v-for="collection in rootCollections"
        :value="collection.id"
        class="border-none p-0 mt-0 outline-none"
    >
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Content Name</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow
              v-for="item in collectionItems"
              :key="item.id"
              @click="router.push(`/collections/${item.id}`)"
              class="cursor-pointer"
          >
            <TableCell class="font-medium flex content-center">
              <NuxtLink :to="'/collections/' + item.id">{{ item.name }}</NuxtLink>
            </TableCell>
          </TableRow>
        </TableBody>
        <TableFooter>
          <TableRow v-if="!collectionItems || collectionItems.length === 0">
            <TableCell class="font-medium flex content-center">
              No content found.
            </TableCell>
          </TableRow>
        </TableFooter>
      </Table>
    </TabsContent>
  </Tabs>
</template>
