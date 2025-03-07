<script setup lang="ts">
import type { HTMLAttributes } from 'vue'
import { cn } from '@/lib/utils'

const props = defineProps<{
  class?: HTMLAttributes['class']
}>()
</script>

<template>
  <div :class="cn('p-6 pt-0 grow', props.class)">
    <div class="flex flex-col gap-y-4">
      <slot />
    </div>
  </div>
</template>
