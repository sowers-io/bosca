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

const props = defineProps<{
  modelValue?: ContentTypeFilter
}>()

const emits = defineEmits<{
  (e: 'update:modelValue', payload: ContentTypeFilter): void
}>()

const modelValue = useVModel(props, 'modelValue', emits, {
  passive: true,
})

const jpg = ref(true)
const png = ref(true)
const webp = ref(true)
const mp4 = ref(true)
const mp3 = ref(true)
const webm = ref(true)
const selected = ref('')

function updateSelected() {
  let s = ''
  if (jpg.value) s += 'JPEG'
  if (png.value) {
    if (s.length > 0) s += ', '
    s += 'PNG'
  }
  if (mp4.value) {
    if (s.length > 0) s += ', '
    s += 'MP4'
  }
  if (mp3.value) {
    if (s.length > 0) s += ', '
    s += 'MP3'
  }
  if (webp.value) {
    if (s.length > 0) s += ', '
    s += 'WEBP'
  }
  if (webm.value) {
    if (s.length > 0) s += ', '
    s += 'WEBM'
  }
  if (s.length === 0) s = 'Select a Content Type'
  selected.value = s
  const f = modelValue.value
  if (
    f &&
    (f.jpg != jpg.value || f.png != png.value || f.mp4 != mp4.value ||
      f.mp3 != mp3.value || f.webp != webp.value || f.webm != webm.value)
  ) {
    updateFilter()
  }
}

function sync() {
  const f = props.modelValue
  if (!f) return
  if (
    f &&
    (f.jpg != jpg.value || f.png != png.value || f.mp4 != mp4.value ||
      f.mp3 != mp3.value || f.webp != webp.value || f.webm != webm.value)
  ) {
    modelValue.value = f
    emits('update:modelValue', modelValue.value)
    jpg.value = f.jpg
    png.value = f.png
    webp.value = f.webp
    mp4.value = f.mp4
    mp3.value = f.mp3
    webm.value = f.webm
    updateSelected()
  }
}

function updateFilter() {
  modelValue.value = {
    jpg: jpg.value,
    png: png.value,
    mp4: mp4.value,
    mp3: mp3.value,
    webp: webp.value,
    webm: webm.value,
  } as unknown as ContentTypeFilter
  emits('update:modelValue', modelValue.value)
}

onMounted(() => {
  updateSelected()
  sync()
})

onUpdated(() => {
  sync()
})

watch([jpg, png, mp4, mp3, webp, webm], updateSelected)
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
      <DropdownMenuCheckboxItem v-model:checked="jpg">
        JPEG
      </DropdownMenuCheckboxItem>
      <DropdownMenuCheckboxItem v-model:checked="png">
        PNG
      </DropdownMenuCheckboxItem>
      <DropdownMenuCheckboxItem v-model:checked="webp">
        WEBP
      </DropdownMenuCheckboxItem>
      <DropdownMenuCheckboxItem v-model:checked="mp4">
        MP4
      </DropdownMenuCheckboxItem>
      <DropdownMenuCheckboxItem v-model:checked="mp3">
        MP3
      </DropdownMenuCheckboxItem>
      <DropdownMenuCheckboxItem v-model:checked="webm">
        WEBM
      </DropdownMenuCheckboxItem>
    </DropdownMenuContent>
  </DropdownMenu>
</template>
