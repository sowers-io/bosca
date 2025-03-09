<script setup lang="ts">
import type { CollectionItem } from '~/lib/bosca/contentcollection'
import { computedAsync } from '@vueuse/core'
import type {
  GuideStepInput,
  GuideStepModuleInput,
  MetadataFragment,
  MetadataInput,
} from '~/lib/graphql/graphql'
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

const client = useBoscaClient()
const router = useRouter()

const selectedId = ref('')
const currentPage = ref(1)
const limit = ref(12)
const offset = computed(() => (currentPage.value - 1) * limit.value)

const { data: collection } = client.collections.findAsyncData({
  attributes: [{
    attributes: [{ key: 'editor.type', value: 'DocumentsAndGuides' }],
  }],
  offset: 0,
  limit: 1,
})

const collectionItems = computedAsync<CollectionItem[]>(async () => {
  if (!collection.value) return []
  const items =
    (await client.collections.list(collection.value[0].id))?.items || []
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
      return 'bosca/v-' + collection.attributes['editor.type'].toLowerCase()
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

async function newDocumentFromTemplate(
  templateId: string,
  templateVersion: number,
  contentType: string,
  attributes: { [key: string]: string },
  parentCollectionId: string,
  title: string,
  categoryIds: string[],
) {
  const templateDocument = await client.metadata.getDocumentTemplate(
    templateId,
    templateVersion,
  )
  if (templateDocument.defaultAttributes) {
    for (const key in templateDocument.defaultAttributes) {
      attributes[key] = templateDocument.defaultAttributes[key]
    }
  }
  const metadata: MetadataInput = {
    parentCollectionId: parentCollectionId,
    name: title,
    contentType: contentType,
    languageTag: 'en',
    attributes: attributes,
    document: {
      templateMetadataId: templateId,
      templateMetadataVersion: templateVersion,
      title: title,
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
    categoryIds: categoryIds,
  }
  return await client.metadata.add(metadata)
}

async function onAddDocument(
  template: MetadataFragment,
  contentType: string,
  attributes: {
    [key: string]: string
  },
  item: CollectionItem,
) {
  const metadataId = await newDocumentFromTemplate(
    template.id,
    template.version,
    contentType,
    attributes,
    item.id,
    'New ' + item.attributes['editor.type'],
    item.categories.map((c) => c.id),
  )
  await client.metadata.setReady(metadataId)
  await router.push(`/content/${metadataId}`)
}

async function onAddGuide(
  template: MetadataFragment,
  contentType: string,
  attributes: {
    [key: string]: string
  },
  item: CollectionItem,
) {
  const templateGuide = await client.metadata.getGuideTemplate(
    template.id,
    template.version,
  )
  const templateDocument = await client.metadata.getDocumentTemplate(
    template.id,
    template.version,
  )
  const steps = []
  for (const templateStep of templateGuide.steps) {
    const modules: GuideStepModuleInput[] = []
    const newStep = {
      modules: modules,
    } as GuideStepInput
    if (templateStep.metadata) {
      newStep.stepMetadataId = await newDocumentFromTemplate(
        templateStep.metadata.id,
        templateStep.metadata.version,
        contentType + '-step',
        {},
        item.id,
        'New Step ' + (steps.length + 1),
        item.categories.map((c) => c.id),
      )
      newStep.stepMetadataVersion = 1
    }
    for (const module of templateStep.modules) {
      if (!module.metadata) continue
      modules.push({
        moduleMetadataId: await newDocumentFromTemplate(
          module.metadata.id,
          module.metadata.version,
          contentType + '-module',
          {},
          item.id,
          'New Step Module ' + (modules.length + 1),
          item.categories.map((c) => c.id),
        ),
        moduleMetadataVersion: 1,
      } as GuideStepModuleInput)
    }
    steps.push(newStep)
  }
  if (templateDocument.defaultAttributes) {
    for (const key in templateDocument.defaultAttributes) {
      attributes[key] = templateDocument.defaultAttributes[key]
    }
  }
  const metadata: MetadataInput = {
    parentCollectionId: item.id,
    name: 'New ' + item.attributes['editor.type'],
    contentType: contentType,
    languageTag: 'en',
    attributes: attributes,
    guide: {
      templateMetadataId: template.id,
      templateMetadataVersion: template.version,
      guideType: templateGuide.type,
      rrule: templateGuide.rrule && templateGuide.rrule.length > 0
        ? templateGuide.rrule
        : null,
      steps: steps,
    },
    document: {
      templateMetadataId: template.id,
      templateMetadataVersion: template.version,
      title: 'New ' + item.attributes['editor.type'],
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
    categoryIds: item.categories.map((c) => c.id),
  }
  const metadataId = await client.metadata.add(metadata)
  await client.metadata.setReady(metadataId)
  await router.push(`/content/${metadataId}`)
}

async function onAdd() {
  for (const item of collectionItems.value || []) {
    if (item.id === selectedId.value) {
      console.log(item)
      const contentType = 'bosca/v-' +
        item.attributes['editor.type'].toLowerCase()
      const attrs: { [key: string]: string } = {
        'editor.type': item.attributes['editor.type'],
      }
      const templates = await client.metadata.find({
        attributes: [],
        contentTypes: [contentType + '-template'],
        categoryIds: categoryIds,
        offset: 0,
        limit: 1,
      })
      const template = templates[0]
      if (!template) {
        toast({
          title: 'No template found (' + contentType + ')',
          description: 'Please create a template first',
        })
        return
      }
      if (item.attributes['editor.type'] === 'Document') {
        await onAddDocument(template, contentType, attrs, item)
      } else if (item.attributes['editor.type'] === 'Guide') {
        await onAddGuide(template, contentType, attrs, item)
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
