<script setup lang="ts">
import mermaid from 'mermaid'
import { nodeTextContent } from '@nuxtjs/mdc/runtime';

const el = ref()
const slots = useSlots()
const content = computed(() => {
  const defaultSlot = slots.default?.()[0]
  if (!defaultSlot) {
    return ''
  }
  return nodeTextContent(defaultSlot)
})

onMounted(async () => {
  try {
    mermaid.initialize({
      startOnLoad: true,
      theme: 'neutral',
      layout: 'elk',
      securityLevel: 'loose'
    })
    await mermaid.run({
      suppressErrors: true,
      nodes: [el.value]
    })
  } catch (e) {
    console.log(e)
  }
  console.log(slots)
})
</script>

<template>
  <div ref="el" class="mermaid">
    {{ content }}
  </div>
</template>
