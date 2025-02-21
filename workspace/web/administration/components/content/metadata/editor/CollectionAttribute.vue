<script lang="ts" setup>
import type { AttributeState } from '~/lib/attribute'
import CollectionItem from "~/components/content/metadata/editor/CollectionItem.vue";
import type { CollectionIdNameFragment } from "~/lib/graphql/graphql.ts";

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
const { data } = client.search.searchAsyncData(
    query,
    filter,
    offset,
    limit,
    storageSystemId || '',
)
</script>

<template>
  <div v-if="attribute">
    <label class="block font-bold mt-4 mb-2">{{ attribute.name }}</label>
    <div class="flex items-center justify-center" v-if="editable">
      <div class="w-full">
        <Combobox
            v-model:search-term="query"
            :display-value="(val: any) => attribute?.value?.name"
            :filter-function="(val: any) => val">
          <ComboboxAnchor as-child class="w-full h-12 bg-background hover:bg-background">
            <ComboboxTrigger as-child>
              <Button variant="outline" class="justify-between text-gray-400 p-3">
                <CollectionItem v-if="attribute.value" :collection="attribute.value" />
                <span v-else>Select an Item...</span>
              </Button>
            </ComboboxTrigger>
          </ComboboxAnchor>
          <ComboboxList>
            <div class="relative w-full max-w-sm items-center">
              <ComboboxInput class="pl-9 focus-visible:ring-0 border-0 border-b rounded-none h-10" placeholder="Search..." />
              <span class="absolute start-0 inset-y-0 flex items-center justify-center px-3">
                <Icon name="i-lucide-search" class="size-4 text-muted-foreground" />
              </span>
            </div>
            <ComboboxEmpty>
              No items found.
            </ComboboxEmpty>
            <ComboboxGroup>
              <ComboboxItem
                  v-for="collection in data"
                  :key="collection.id!"
                  :value="collection.id!"
                  class="cursor-pointer"
                  @click="(e: any) => { attribute!.value = collection; e.preventDefault(); }"
              >
                <CollectionItem :collection="collection as CollectionIdNameFragment" />
                <ComboboxItemIndicator>
                  <Icon name="i-lucide-check" class="size-4 text-success-foreground" />
                </ComboboxItemIndicator>
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
