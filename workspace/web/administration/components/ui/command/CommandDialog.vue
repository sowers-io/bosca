<script setup lang="ts">
import type { DialogRootProps } from 'radix-vue'
import { useForwardPropsEmits, VisuallyHidden } from 'radix-vue'
import { Dialog, DialogContent } from '../dialog'
import Command from './Command.vue'

export interface CommandDialogProps extends DialogRootProps {
  search?: string
}

export type DialogRootEmits = {
  'update:search': [value: string]
  'update:open': [value: boolean]
}

const props = defineProps<CommandDialogProps>()
const emits = defineEmits<DialogRootEmits>()

const forwarded = useForwardPropsEmits(props, emits)

const modelValue = useVModel(props, 'search', emits, {
  passive: true,
  defaultValue: props.search,
})

function filter(val: any, term: string): any {
  modelValue.value = term
  emits('update:search', term)
  return val
}
</script>

<template>
  <Dialog v-bind="forwarded">
    <DialogContent class="overflow-hidden p-0 shadow-lg">
      <VisuallyHidden as-child>
        <DialogTitle />
      </VisuallyHidden>
      <VisuallyHidden as-child>
        <DialogDescription aria-describedby="undefined" />
      </VisuallyHidden>
      <Command
        :filter-function="filter"
        :search="modelValue"
        class="[&_[cmdk-input-wrapper]_svg]:h-5 [&_[cmdk-input-wrapper]_svg]:w-5 [&_[cmdk-input]]:h-12 [&_[cmdk-item]_svg]:h-5 [&_[cmdk-item]_svg]:w-5 [&_[cmdk-group-heading]]:px-2 [&_[cmdk-group]]:px-2 [&_[cmdk-item]]:px-2 [&_[cmdk-item]]:py-3 [&_[cmdk-group]:not([hidden])_~[cmdk-group]]:pt-0 [&_[cmdk-group-heading]]:text-muted-foreground [&_[cmdk-group-heading]]:font-medium"
      >
        <slot />
      </Command>
    </DialogContent>
  </Dialog>
</template>
