<script setup lang="ts">
import type {
  CollectionIdNameFragment,
  MetadataIdNameFragment,
  ProfileIdNameFragment,
} from '~/lib/graphql/graphql'
import { getLink } from '~/lib/link.ts'
import { toast } from '~/components/ui/toast'
import { Command } from '~/components/ui/command'

const client = useBoscaClient()
const openCommand = ref(false)
const router = useRouter()
const { data: storageSystems, refresh } = await client.workflows
  .getStorageSystemsAsyncData()

const search = ref('')
const offset = ref(0)
const limit = ref(10)
const filter = ref<string | null>(null)
const storageSystemId = computed(() =>
  storageSystems.value?.find((s) => s.name === 'Default Search')?.id || ''
)

watch(search, () => {
  if (!storageSystemId.value) {
    toast({ title: 'No default storage system found.' })
  }
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
  item:
    | CollectionIdNameFragment
    | MetadataIdNameFragment
    | ProfileIdNameFragment,
) {
  router.push(getLink(item))
  openCommand.value = false
}

onMounted(() => {
  refresh()
})
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
      </div>
    </Button>
  </SidebarMenuButton>

  <CommandDialog v-model:open="openCommand" :filter-function="(val) => val">
    <CommandInput
      v-model:model-value="search"
      placeholder="Type a command or search..."
    />
    <CommandList>
      <CommandEmpty>No results found.</CommandEmpty>
      <CommandGroup heading="Search Results">
        <CommandItem
          v-for="result in results || []"
          :key="result.id || ''"
          :value="result.name || ''"
          class="gap-2 cursor-pointer"
          @click="onResultSelected(result)"
        >
          <ContentListItem :item="result" />
        </CommandItem>
      </CommandGroup>
    </CommandList>
  </CommandDialog>
</template>
