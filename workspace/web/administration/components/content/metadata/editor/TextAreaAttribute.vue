<script lang="ts" setup>
import type { DocumentTemplateAttribute } from '~/lib/graphql/graphql'

defineProps<{
  value: string
  attribute: DocumentTemplateAttribute
  editable: boolean
  loading: boolean
  onChange: (attribute: DocumentTemplateAttribute, value: any) => void
  onRunWorkflow: (attribute: DocumentTemplateAttribute) => void
}>()
</script>

<template>
  <div>
    <label class="block font-bold mt-4 mb-2">{{ attribute.name }}</label>
    <div class="flex items-center justify-center">
      <Textarea
        class="w-full border rounded-md p-2"
        :value="value"
        @input="(e: any) => onChange(attribute, e.target!.value)"
        :disabled="!editable"
      />
      <Tooltip v-if="editable">
        <TooltipTrigger as-child>
          <Button
            v-if="attribute.workflows && attribute.workflows.length"
            :disabled="loading"
            class="flex items-center justify-center ms-2 size-8 p-0"
            variant="ghost"
            @click="onRunWorkflow(attribute)"
          >
            <Icon name="i-lucide-sparkles" class="size-4" v-if="!loading" />
            <Icon name="i-lucide-loader-circle" class="size-4 animate-spin" v-else />
          </Button>
        </TooltipTrigger>
        <TooltipContent>
          <p>{{ attribute.description }}</p>
        </TooltipContent>
      </Tooltip>
    </div>
  </div>
</template>
