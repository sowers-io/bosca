<script setup lang="ts">
import type { CollectionItem } from '~/lib/bosca/contentcollection'
import { computedAsync } from '@vueuse/core'
import type { MetadataInput } from '~/lib/graphql/graphql'
import TableFooter from '~/components/ui/table/TableFooter.vue'
import {toast} from "~/components/ui/toast";

const client = useBoscaClient()
const router = useRouter()

const selectedId = ref('')
const selectedType = ref('items')
const offset = ref(0)
const limit = ref(10)

const { data: collection } = client.collections.findAsyncData({
  attributes: [ { attributes: [ { key: 'editor.type', value: 'DocumentsAndGuides' } ] } ],
  offset: 0,
  limit: 1,
})

const collections = computedAsync<CollectionItem[]>(async () => {
  if (!collection.value) return []
  const items = (await client.collections.list(collection.value[0].id))?.items || []
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

const { data: items } = client.metadata.findAsyncData({
  attributes: [],
  contentTypes: ['bosca/v-document'],
  categoryIds: categoryIds,
  offset: offset,
  limit: limit,
})

const { data: templates } = client.metadata.findAsyncData({
  attributes: [],
  contentTypes: ['bosca/v-document-template'],
  categoryIds: categoryIds,
  offset: 0,
  limit: 1,
})

const content = computed(() => {
  return items.value?.filter((i) =>
    i.attributes && !i.attributes['template.type']
  ) || []
})

async function onAdd() {
  for (const collection of collections.value || []) {
    if (collection.id === selectedId.value) {
      const attrs: { [key: string]: string } = {}
      let ct = 'bosca/v-' + collection.attributes['editor.type'].toLowerCase()
      if (selectedType.value === 'templates') {
        attrs['editor.type'] = 'Template'
        attrs['template.type'] = collection.attributes['editor.type']
        ct += '-template'
      } else {
        attrs['editor.type'] = collection.attributes['editor.type']
      }
      const template = templates.value ? templates.value[0] : null
      if (!template) {
        toast({
          title: 'No template found',
          description: 'Please create a template first'
        })
        return
      }
      const templateDocument = await client.metadata.getDocumentTemplate(
        template.id,
        template.version,
      )
      if (templateDocument.defaultAttributes) {
        for (const key in templateDocument.defaultAttributes) {
          attrs[key] = templateDocument.defaultAttributes[key]
        }
      }
      const metadata: MetadataInput = {
        parentCollectionId: collection.id,
        name: 'New ' + collection.attributes['editor.type'],
        contentType: ct,
        languageTag: 'en',
        attributes: attrs,
        document: {
          templateMetadataId: template.id,
          templateMetadataVersion: template.version,
          title: 'New ' + collection.attributes['editor.type'],
          content: {
            document: templateDocument.content.document,
          },
        },
        profiles: [
          {
            profileId: (await client.profiles.getCurrentProfile()).id!,
            relationship: 'author',
          },
        ],
        categoryIds: collection.categories.map((c) => c.id),
      }
      const metadataId = await client.metadata.add(metadata)
      await client.metadata.setReady(metadataId)
      if (selectedType.value === 'templates') {
        await router.push(`/content/template/${metadataId}`)
      } else {
        await router.push(`/content/${metadataId}`)
      }
      break
    }
  }
}

const breadcrumbs = useBreadcrumbs()

onMounted(() => {
  breadcrumbs.set([{ title: 'Content' }])
})
</script>

<template>
  <Tabs class="h-full space-y-6" v-model:model-value="selectedId">
    <div class="flex">
      <TabsList>
        <TabsTrigger v-for="collection in collections" :value="collection.id">
          {{ collection.name }}
        </TabsTrigger>
      </TabsList>
      <div class="grow"></div>
      <div>
        <Button @click="onAdd">
          <Icon name="i-lucide-plus" />
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
            v-for="item in content"
            :key="item.id"
            @click="router.push(`/content/${item.id}`)"
            class="cursor-pointer"
          >
            <TableCell class="font-medium flex content-center">
              <NuxtLink :to="'/content/' + item.id">{{ item.name }}</NuxtLink>
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
