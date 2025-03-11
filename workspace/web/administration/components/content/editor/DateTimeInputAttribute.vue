<script lang="ts" setup>
import type { AttributeState } from '~/lib/attribute.ts'

defineProps<{
  attribute: AttributeState
  editable: boolean
  workflowsEnabled: boolean
  onRunWorkflow: (attribute: AttributeState) => void
}>()
</script>

<template>
  <div v-if="attribute">
    <label class="block font-bold mt-4 mb-2">{{ attribute.name }}</label>
    <div class="flex items-center justify-center">
      <div class="w-full">
        <Input
          class="w-full border rounded-md p-2 shadow-sm"
          v-model:model-value="attribute.dateTimeValue"
          :disabled="!editable"
        />
        <div class="text-xs text-red py-2 ps-0.5" v-if="attribute.invalidValue">
          Invalid Date/Time
        </div>
        <div
          class="text-xs text-orange py-2 ps-0.5"
          v-if="attribute.valueWarning"
        >
          {{ attribute.valueWarning }}
        </div>
      </div>
      <Tooltip v-if="editable && attribute.hasWorkflows">
        <TooltipTrigger as-child>
          <Button
            :disabled="attribute.loading || !workflowsEnabled"
            class="flex items-center justify-center ms-2 size-8 p-0"
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
  </div>
</template>
