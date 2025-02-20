<script lang="ts" setup>
import type {
  DocumentTemplateAttribute,
  MetadataProfile,
  MetadataProfileInput,
} from '~/lib/graphql/graphql'
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from '@/components/ui/command'

import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover'
import { Check } from 'lucide-vue-next'
import { ref } from 'vue'

const client = useBoscaClient()
const query = ref('')
const filter = ref('_type = "profile"')
const offset = ref(0)
const limit = ref(50)
const storageSystemId = (await client.workflows.getStorageSystems()).find((s) =>
  s.name === 'Default Search'
)?.id
const { data: profiles } = client.search.searchAsyncData(
  query,
  filter,
  offset,
  limit,
  storageSystemId || '',
)

const props = defineProps<{
  profile: MetadataProfile | null
  attribute: DocumentTemplateAttribute
  editable: boolean
  onChange: (
    attribute: DocumentTemplateAttribute,
    profile: MetadataProfileInput | null,
  ) => void
}>()

const open = ref(false)
const value = ref('')

function onSelect(id: string) {
  props.onChange(props.attribute, {
    profileId: id,
    relationship: 'author',
  })
  open.value = false
}
</script>

<template>
  <Popover v-model:open="open">
    <PopoverTrigger as-child>
      <label class="block font-bold mt-4 mb-2">{{ attribute.name }}</label>
      <div
        class="flex items-center gap-4 rounded-md border border-input bg-background px-3 py-2 cursor-pointer"
        @click="open = true"
      >
        <Avatar>
          <AvatarImage
            :src="'/content/image?id=' + profile?.profile?.attributes?.find(a => a.typeId === 'bosca.profiles.avatar')?.metadata?.id || '#'"
            :alt="profile?.profile?.name || 'Select a Profile'"
          />
          <AvatarFallback>{{
            profile?.profile?.name?.substring(0, 1)
            ?.toLocaleUpperCase() || '-'
          }}</AvatarFallback>
        </Avatar>
        {{ profile?.profile?.name || 'Select a Profile' }}
      </div>
      <div class="grid justify-items-end" v-if="editable && profile">
        <Button variant="ghost" @click="onChange(attribute, null)">
          Clear
        </Button>
      </div>
    </PopoverTrigger>
    <PopoverContent class="w-[200px] p-0">
      <Command v-model:search-term="query" :filter-function="(val) => val">
        <CommandInput class="h-9" placeholder="Search for a Profile..." />
        <CommandEmpty>No profiles found.</CommandEmpty>
        <CommandList>
          <CommandGroup>
            <CommandItem
              v-for="p in profiles || []"
              :key="p.id!"
              :value="p.id!"
              @select="onSelect(p.id!)"
            >
              {{ p.name }}
              <Check
                :class="
                  cn(
                    'ml-auto h-4 w-4',
                    value === profile?.profile?.id
                      ? 'opacity-100'
                      : 'opacity-0',
                  )
                "
              />
            </CommandItem>
          </CommandGroup>
        </CommandList>
      </Command>
    </PopoverContent>
  </Popover>
</template>
