<script lang="ts" setup>
const route = useRoute()
const client = useBoscaClient()
const { data: list, refresh } = client.collections.listAsyncData(
  route.params.id.toString(),
)

client.listeners.onCollectionChanged((id) => {
  if (list.value?.collection?.id === id) {
    refresh()
  }
})

const breadcrumbs = useBreadcrumbs()
const title = computed(() => list.value?.collection?.name || 'Collection')

onMounted(() => {
  const links: BreadcrumbLink[] = [
    route.query.bible ? { title: 'Bible', to: '/bible' } : {
      title: 'Collections',
      to: '/collections/browse' +
        (route.query.parent ? '?id=' + route.query.parent : ''),
    },
  ]
  if (route.query.parent) {
    links.push({ title: '...' })
  }
  links.push({ title: title })
  links.push({ title: 'Edit' })
  breadcrumbs.set(links)
})
</script>
<template>
  <Tabs
    default-value="general"
    class="h-full space-y-6"
    v-if="list?.collection"
  >
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
      <TabsTrigger value="supplementary">
        Supplementary
      </TabsTrigger>
      <TabsTrigger value="items">
        Items
      </TabsTrigger>
      <TabsTrigger value="workflows">
        Workflows
      </TabsTrigger>
    </TabsList>
    <TabsContent value="general" class="border-none p-0 outline-none">
      <div class="flex flex-wrap gap-4 m-auto justify-center">
        <ContentCollectionsDetails
          :collection="list.collection"
          class="w-[600px]"
        />
        <ContentVisibility :content="list.collection" class="w-[600px]" />
        <ContentStates :content="list.collection" class="w-[600px]" />
        <ContentContents :content="list.collection" class="w-[600px]" />
      </div>
    </TabsContent>
    <TabsContent value="traits" class="border-none p-0 outline-none">
      <ContentTraits class="col-span-2" :content="list.collection" />
    </TabsContent>
    <TabsContent value="attributes" class="border-none p-0 outline-none">
      <ContentAttributes class="col-span-2" :content="list.collection" />
    </TabsContent>
    <TabsContent value="supplementary" class="border-none p-0 outline-none">
      <ContentCollectionsSupplementary class="col-span-2" :collection="list.collection" />
    </TabsContent>
    <TabsContent value="items" class="border-none p-0 outline-none">
      <ContentCollectionsItems
        class="col-span-2"
        :collection="list.collection"
        :items="list.items"
      />
    </TabsContent>
    <TabsContent value="workflows" class="border-none p-0 outline-none">
      <ContentWorkflows class="col-span-2" :content="list.collection" />
    </TabsContent>
  </Tabs>
</template>
