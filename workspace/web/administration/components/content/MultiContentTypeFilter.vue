<script lang="ts" setup>
import { Button } from '@/components/ui/button'
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import type { ContentTypeFilter } from '~/lib/bosca/contentmetadata'
import type {Reactive} from "vue";

const props = defineProps<{
  filter: Reactive<ContentTypeFilter>
}>()

const selected = ref('')

function updateSelected() {
  let s = ''
  if (props.filter.jpg) s += 'JPEG'
  if (props.filter.png) {
    if (s.length > 0) s += ', '
    s += 'PNG'
  }
  if (props.filter.mp4) {
    if (s.length > 0) s += ', '
    s += 'MP4'
  }
  if (props.filter.mp3) {
    if (s.length > 0) s += ', '
    s += 'MP3'
  }
  if (props.filter.webp) {
    if (s.length > 0) s += ', '
    s += 'WEBP'
  }
  if (props.filter.webm) {
    if (s.length > 0) s += ', '
    s += 'WEBM'
  }
  if (props.filter.youtube) {
    if (s.length > 0) s += ', '
    s += 'YouTube'
  }
  if (s.length === 0) s = 'Select a Content Type'
  selected.value = s
}

onMounted(() => {
  updateSelected()
})

onUpdated(() => {
  updateSelected()
})

watch([props.filter], updateSelected)
</script>

<template>
  <DropdownMenu>
    <DropdownMenuTrigger as-child>
      <Button variant="outline">
        {{ selected }}
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent class="w-56">
      <DropdownMenuLabel>Content Types</DropdownMenuLabel>
      <DropdownMenuSeparator />
      <DropdownMenuCheckboxItem v-model:checked="filter.jpg">
        JPEG
      </DropdownMenuCheckboxItem>
      <DropdownMenuCheckboxItem v-model:checked="filter.png">
        PNG
      </DropdownMenuCheckboxItem>
      <DropdownMenuCheckboxItem v-model:checked="filter.webp">
        WEBP
      </DropdownMenuCheckboxItem>
      <DropdownMenuCheckboxItem v-model:checked="filter.mp4">
        MP4
      </DropdownMenuCheckboxItem>
      <DropdownMenuCheckboxItem v-model:checked="filter.mp3">
        MP3
      </DropdownMenuCheckboxItem>
      <DropdownMenuCheckboxItem v-model:checked="filter.webm">
        WEBM
      </DropdownMenuCheckboxItem>
      <DropdownMenuCheckboxItem v-model:checked="filter.youtube">
        YouTube
      </DropdownMenuCheckboxItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>
