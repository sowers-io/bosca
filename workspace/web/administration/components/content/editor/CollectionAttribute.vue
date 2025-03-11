<script lang="ts" setup>
import type { AttributeState } from '~/lib/attribute.ts'
import CollectionItem from '~/components/content/editor/CollectionItem.vue'
import type { CollectionIdNameFragment } from '~/lib/graphql/graphql.ts'

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
const limit = ref(15)
const storageSystemId = (await client.workflows.getStorageSystems()).find((s) =>
  s.name === 'Default Search'
)?.id
const open = ref(false)
const { data } = client.search.searchAsyncData(
  query,
  filter,
  offset,
  limit,
  storageSystemId || '',
)

function onSelect(id: CollectionIdNameFragment) {
  props.attribute!.value = id
  query.value = ''
}
</script>

<template>
  <div v-if="attribute">
    <label class="block font-bold mt-4 mb-2">{{ attribute.name }}</label>
    <div class="flex items-center justify-center" v-if="editable">
      <div class="w-full">
        <Combobox
          class="w-full cursor-pointer"
          v-model:open="open"
          v-model="attribute.value"
          @click="open = true"
          :ignore-filter="true"
          :reset-search-term-on-select="true"
          :filter-function="(val: any) => val"
        >
          <ComboboxAnchor class="w-full">
            <div
              class="relative w-full items-center bg-background border shadow-sm rounded-md h-14"
            >
              <CollectionItem
                :collection="attribute.value"
                class="p-3"
                v-if="!open && attribute.value"
              />
              <ComboboxInput
                v-model="query"
                class="w-full h-full p-4 border-none shadow-none"
                placeholder="Select an Item..."
                v-else
              />
            </div>
          </ComboboxAnchor>
          <ComboboxList class="w-[--reka-popper-anchor-width]">
            <ComboboxEmpty>
              No items found.
            </ComboboxEmpty>
            <ComboboxGroup>
              <ComboboxItem
                v-for="item in data"
                :key="item.id"
                :value="item"
                @click.prevent="onSelect(item as CollectionIdNameFragment)"
              >
                <CollectionItem
                  :collection="item as CollectionIdNameFragment"
                />
              </ComboboxItem>
            </ComboboxGroup>
          </ComboboxList>
        </Combobox>
        <div class="grid justify-items-end" v-if="editable && attribute.value">
          <Button variant="ghost" @click="attribute.value = null">
            Clear
          </Button>
        </div>
      </div>
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
    <div v-else-if="attribute.value">
      <CollectionItem :collection="attribute.value" />
    </div>
    <div v-else>
      <span class="text-muted-foreground">No Selection</span>
    </div>
  </div>
</template>
