<script lang="ts" setup>
// Import styles.
import 'vidstack/player/styles/default/theme.css'
import 'vidstack/player/styles/default/layouts/audio.css'
import 'vidstack/player/styles/default/layouts/video.css'
// Register elements.
import 'vidstack/player'
import 'vidstack/player/layouts'
import 'vidstack/player/ui'

import type { MediaPlayerElement } from 'vidstack/elements'
import { onMounted, ref } from 'vue'
import type { MetadataFragment } from '~/lib/graphql/graphql.ts'

const props = defineProps<{
  metadata: MetadataFragment | null
}>()

const $player = ref<MediaPlayerElement>()
const $src = ref(
  props.metadata?.content?.type === 'bosca/x-youtube-video'
    ? 'youtube/' + props.metadata?.attributes['youtube.id']
    : '/content/file?id=' + props.metadata?.id,
)

function updateSource() {
  let url = props.metadata?.content?.type === 'bosca/x-youtube-video'
    ? 'youtube/' + props.metadata?.attributes['youtube.id']
    : '/content/file?id=' + props.metadata?.id
  if (url && url != $player.value!.src) {
    $src.value = url
  }
}

onUpdated(() => {
  updateSource()
})

onMounted(() => {
  updateSource()
})

onBeforeUnmount(() => {
  // This call will destroy the player and all child instances.
  $player.value?.destroy()
})
</script>

<template>
  <media-player
    v-if="$src"
    keep-alive
    class="player"
    :title="metadata?.name"
    :src="$src"
    ref="$player"
  >
    <media-provider></media-provider>
    <media-audio-layout />
    <media-video-layout />
  </media-player>
  <div v-else>...</div>
</template>

<style scoped>
.player .vds-audio-layout {
  filter: none;
  @apply border border-input;
}

.player[data-view-type='audio'] media-poster {
  display: none;
}

.player[data-view-type='video'] {
  aspect-ratio: 16 / 9;
}

.src-buttons {
  display: flex;
  align-items: center;
  justify-content: space-evenly;
  margin-top: 40px;
  margin-inline: auto;
  max-width: 300px;
}
</style>
