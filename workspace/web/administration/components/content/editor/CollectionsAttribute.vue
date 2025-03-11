<script lang="ts" setup>
import type { CollectionIdNameFragment } from '~/lib/graphql/graphql.ts'

import {
  TagsInput,
  TagsInputInput,
  TagsInputItem,
  TagsInputItemDelete,
  TagsInputItemText,
} from '~/components/ui/tags-input'
import { computed, ref } from 'vue'
import type { AttributeState } from '~/lib/attribute.ts'

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
    (props.attribute?.configuration?.searchFilter
      ? ' AND ' + props.attribute?.configuration?.searchFilter
      : ''),
)
const offset = ref(0)
const limit = ref(50)
const storageSystemId = (await client.workflows.getStorageSystems()).find((s) =>
  s.name === 'Default Search'
)?.id
const { data } = client.search.searchAsyncData(
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
    .filter((c) => c.id !== id.id)
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
    <div class="flex items-center justify-center w-full" v-if="editable">
      <Combobox
        v-model="attribute.value"
        v-model:open="open"
        :ignore-filter="true"
        :reset-search-term-on-select="true"
        :filter-function="(val: any) => val"
        class="w-full"
      >
        <ComboboxAnchor as-child class="w-full" @click.prevent="open = true">
          <TagsInput
            v-model="attribute.value"
            class="px-2 gap-2 w-full shadow-sm"
          >
            <div>
              <div class="flex gap-2 flex-wrap items-center">
                <TagsInputItem
                  v-for="item in attribute.value"
                  :key="item.id"
                  :value="item.name"
                  class="h-7 bg-primary text-secondary"
                >
                  <TagsInputItemText />
                  <TagsInputItemDelete @click.prevent="onRemove(item)" />
                </TagsInputItem>
              </div>
              <div>
                <ComboboxInput v-model="query" as-child class="w-full block">
                  <TagsInputInput
                    placeholder="Select Items..."
                    class="min-w-[200px] w-full p-2 border-none shadow-none focus-visible:ring-0 h-auto"
                    @keydown.enter.prevent
                  />
                </ComboboxInput>
              </div>
            </div>
          </TagsInput>

          <ComboboxList class="w-[--reka-popper-anchor-width]">
            <ComboboxEmpty />
            <ComboboxGroup>
              <ComboboxItem
                v-for="item in data"
                :key="item.id"
                :value="item.name"
                @select.prevent="
                  (ev) => {
                    onSelect(item as CollectionIdNameFragment)
                    open = false
                  }
                "
              >
                {{ item.name }}
              </ComboboxItem>
            </ComboboxGroup>
          </ComboboxList>
        </ComboboxAnchor>
      </Combobox>
      <Tooltip v-if="editable && attribute.hasWorkflows">
        <TooltipTrigger as-child>
          <Button
            class="flex items-center justify-center ms-2 size-8 p-0"
            :disabled="attribute.loading || !workflowsEnabled"
            variant="ghost"
            @click="onRunWorkflow(attribute)"
          >
            <Icon
              name="i-lucide-sparkles"
              class="size-4"
              v-if="!attribute.loading"
            />
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
    <div v-else-if="attribute.value && attribute.value.length > 0">
      <Badge
        v-for="(item, index) in attribute.value"
        variant="outline"
        class="mr-2"
      >
        {{ item.name }}
      </Badge>
    </div>
    <div v-else>
      No Selection
    </div>
  </div>
</template>
