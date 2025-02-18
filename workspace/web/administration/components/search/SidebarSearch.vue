<script setup lang="ts">
import type {
  CollectionIdNameFragment,
  MetadataIdNameFragment,
} from '~/lib/graphql/graphql'

const { metaSymbol } = useShortcuts()
const client = useBoscaClient()
const openCommand = ref(false)
const router = useRouter()
const { data: storageSystems } = await client.workflows
  .getStorageSystemsAsyncData()

const search = ref('')
const offset = ref(0)
const limit = ref(10)
const filter = ref<string | null>(null)
const storageSystemId = ref(
  storageSystems.value?.find((s) => s.name === 'Default Search')?.id || '',
)

watch(search, () => {
  if (search.value.length > 0) {
    if (limit.value == 10) {
      limit.value = 100
    }
  } else {
    if (limit.value != 10) {
      limit.value = 10
    }
  }
})

const { data: results } = await client.search.searchAsyncData(
  search,
  filter,
  offset,
  limit,
  storageSystemId,
)

defineShortcuts({
  Meta_K: () => openCommand.value = true,
})

function onResultSelected(
  item: CollectionIdNameFragment | MetadataIdNameFragment,
) {
  if (item.version) {
    router.push('/metadata/edit/' + item.id)
  } else {
    router.push('/collections/edit/' + item.id)
  }
  openCommand.value = false
}
</script>

<template>
  <SidebarMenuButton as-child tooltip="Search">
    <Button
      variant="outline"
      size="sm"
      class="text-xs"
      @click="openCommand = !openCommand"
    >
      <Icon name="i-lucide-search" />
      <span
        class="font-normal text-gray-400 group-data-[collapsible=icon]:hidden"
      >Search</span>
      <div
        class="ml-auto flex items-center space-x-0.5 group-data-[collapsible=icon]:hidden"
      >
        <BaseKbd>{{ metaSymbol }}</BaseKbd>
        <BaseKbd>K</BaseKbd>
      </div>
    </Button>
  </SidebarMenuButton>

  <CommandDialog v-model:open="openCommand" v-model:search="search">
    <CommandInput placeholder="Type a command or search..." />
    <CommandList>
      <CommandEmpty>No results found.</CommandEmpty>
      <CommandGroup heading="Suggestions"> </CommandGroup>
      <CommandSeparator />
      <CommandGroup heading="Search Results">
        <CommandItem
          v-for="result in results || []"
          :key="result.id"
          :value="result.name"
          class="gap-2 cursor-pointer"
          @select="onResultSelected(result)"
        >
          {{ result.name }}
        </CommandItem>
      </CommandGroup>
    </CommandList>
  </CommandDialog>
</template>

<style scoped>
</style>
