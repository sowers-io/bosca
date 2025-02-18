<script setup lang="ts">
import { cn } from '@/lib/utils'
import type { MetadataFragment } from '~/lib/graphql/graphql'

interface MetadataImageProps {
  metadata: MetadataFragment
  aspectRatio?: 'portrait' | 'square'
  width?: number
  height?: number
  onSelected?: (id: string) => void
}

const router = useRouter()
const props = withDefaults(defineProps<MetadataImageProps>(), {
  aspectRatio: 'portrait',
})

function onClick() {
  if (props.onSelected) {
    props.onSelected(props.metadata.id)
  } else {
    router.push('/metadata/edit/' + props.metadata.id + '?media=true')
  }
}

const imageUrl = computed(() => {
  if (props.metadata.content.type.startsWith('image/')) {
    return props.metadata.content.urls.download.url
  }
  const thumbnails = props.metadata.supplementary.filter((s) =>
    s.key.startsWith('thumbnail')
  )
  return thumbnails.length > 0
    ? thumbnails[Math.floor(thumbnails.length / 2)].content.urls.download.url
    : null
})
</script>

<template>
  <div
    :class="cn('space-y-3 cursor-pointer', $attrs.class ?? '')"
    @click="onClick"
  >
    <div class="overflow-hidden rounded-md">
      <img
        v-if="imageUrl"
        :src="imageUrl"
        :alt="metadata.name"
        :width="width"
        :height="height"
        :class="
          cn(
            'h-auto bg-gray-100 dark:bg-slate-800 w-auto object-cover transition-all hover:scale-105',
            aspectRatio === 'portrait' ? 'aspect-[3/4]' : 'aspect-square',
          )
        "
      />
      <div
        v-else
        :class="
          cn(
            'h-auto bg-gray-100 dark:bg-slate-800 w-auto object-cover transition-all hover:scale-105',
            aspectRatio === 'portrait' ? 'aspect-[3/4]' : 'aspect-square',
          )
        "
      >
      </div>
    </div>
    <div class="space-y-1 text-sm">
      <h3 class="font-medium leading-none">
        {{ metadata.name }}
      </h3>
    </div>
  </div>
</template>
