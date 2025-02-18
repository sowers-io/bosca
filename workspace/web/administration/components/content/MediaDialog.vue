<script setup lang="ts">
defineProps<{
  open: boolean
  onSelected: (metadataId: string) => void
}>()

const open = ref(false)
</script>

<template>
  <Dialog :open="open">
    <DialogContent
      class="h-[calc(100dvh-100px)] w-[calc(100dvw-100px)] max-w-full overflow-auto z-100"
    >
      <div class="flex flex-col gap-2 h-full">
        <ContentMedia :on-selected="onSelected" />
      </div>
    </DialogContent>
  </Dialog>
</template>
