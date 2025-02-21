<script lang="ts" setup>
import type {
  CollectionIdNameFragment,
  DocumentTemplateAttribute,
  MetadataProfile,
  MetadataProfileInput,
} from '~/lib/graphql/graphql'

import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandItem,
  CommandList,
} from '@/components/ui/command'
import {
  TagsInput,
  TagsInputInput,
  TagsInputItem,
  TagsInputItemDelete,
  TagsInputItemText,
} from '@/components/ui/tags-input'
import {
  ComboboxAnchor,
  ComboboxContent,
  ComboboxInput,
  ComboboxPortal,
  ComboboxRoot,
} from 'radix-vue'
import {computed, ref} from 'vue'
import type {AttributeState} from "~/lib/attribute.ts";

const props = defineProps<{
  attribute: AttributeState | undefined | null
  editable: boolean
  workflowsEnabled: boolean
  onRunWorkflow: (attribute: AttributeState) => void
}>()

const client = useBoscaClient()
const query = ref('')
const filter = ref(
    '_type = "collection"' +
    (props.attribute?.configuration?.filter
        ? ' AND ' + props.attribute?.configuration?.filter
        : ''),
)
const offset = ref(0)
const limit = ref(50)
const storageSystemId = (await client.workflows.getStorageSystems()).find((s) =>
    s.name === 'Default Search'
)?.id
const {data} = client.search.searchAsyncData(
    query,
    filter,
    offset,
    limit,
    storageSystemId || '',
)

const open = ref(false)

function onSelect(id: CollectionIdNameFragment) {
  const attr = props.attribute
  if (!attr) return
  const newCollections = [...(attr.value as CollectionIdNameFragment[] || [])]
  newCollections.push(id)
  attr.value = newCollections
  open.value = false
}

function onRemove(id: CollectionIdNameFragment) {
  const attr = props.attribute
  if (!attr) return
  attr.value = attr.value.filter((c: any) => c.id != id.id)
  open.value = false
}
</script>

<template>
  <div v-if="attribute">
    <label class="block font-bold mt-4 mb-2">{{ attribute.name }}</label>
    <div class="flex items-center justify-center">
      <TagsInput class="px-0 min-h-10 py-2 gap-0 w-full" v-model="attribute.value">
        <div class="flex gap-2 flex-wrap items-center px-3">
          <TagsInputItem v-for="(item, index) in attribute.value" :value="item.name">
            <TagsInputItemText/>
            <TagsInputItemDelete @click="onRemove(item)"/>
          </TagsInputItem>
        </div>
        <ComboboxRoot
            v-model:open="open"
            v-model:search-term="query"
            class="w-full"
            :filter-function="(val) => val"
        >
          <ComboboxAnchor as-child>
            <ComboboxInput placeholder="Topics..." as-child>
              <TagsInputInput
                  class="w-full px-3"
                  :class="attribute.value.length > 0 ? 'mt-2' : ''"
                  @keydown.enter.prevent
              />
            </ComboboxInput>
          </ComboboxAnchor>
          <ComboboxPortal>
            <ComboboxContent>
              <CommandList
                  position="popper"
                  class="w-[--radix-popper-anchor-width] rounded-md mt-2 border bg-popover text-popover-foreground shadow-md outline-none data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2"
              >
                <CommandEmpty/>
                <CommandGroup>
                  <CommandItem
                      v-for="c in data || []"
                      :key="c.id!"
                      :value="c.id!"
                      @select.prevent="
                    (ev) => {
                      onSelect(c as CollectionIdNameFragment)
                      query = ''
                    }
                  "
                  >
                    {{ c.name }}
                  </CommandItem>
                </CommandGroup>
              </CommandList>
            </ComboboxContent>
          </ComboboxPortal>
        </ComboboxRoot>
      </TagsInput>
      <Tooltip v-if="editable && attribute.hasWorkflows">
        <TooltipTrigger as-child>
          <Button
              class="flex items-center justify-center ms-2 size-8 p-0"
              :disabled="attribute.loading || !workflowsEnabled"
              variant="ghost"
              @click="onRunWorkflow(attribute)"
          >
            <Icon name="i-lucide-sparkles" class="size-4" v-if="!attribute.loading"/>
            <Icon
                name="i-lucide-loader-circle"
                class="size-4 animate-spin"
                v-else
            />
          </Button>
        </TooltipTrigger>
        <TooltipContent>
          <p>{{ attribute.description }}</p>
        </TooltipContent>
      </Tooltip>
    </div>
  </div>
</template>
