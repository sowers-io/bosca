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
import { Check, ChevronsUpDown } from 'lucide-vue-next'
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

defineProps<{
  profile: MetadataProfile | null
  attribute: DocumentTemplateAttribute
  editable: boolean
  onChange: (
    attribute: DocumentTemplateAttribute,
    profile: MetadataProfileInput,
  ) => void
}>()

const open = ref(false)
const value = ref('')
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
            src="https://github.com/radix-vue.png"
            alt="@radix-vue"
          />
          <AvatarFallback>{{
            profile?.profile?.name?.substring(0, 1)
            ?.toLocaleUpperCase() || '-'
          }}</AvatarFallback>
        </Avatar>
        {{ profile?.profile?.name || 'Select a Profile' }}
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
              @select="
                ;((ev) => {
                  onChange(attribute, {
                    profileId: p.id!,
                    relationship: 'author',
                  })
                  open = false
                })
              "
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
