<script setup lang="ts">
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '~/components/ui/card'
import type { MetadataFragment } from '~/lib/graphql/graphql.ts'

defineProps<{
  metadata: MetadataFragment
}>()
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Content</CardTitle>
      <CardDescription>View the content</CardDescription>
    </CardHeader>
    <CardContent>
      <template v-if="metadata.content.type.startsWith('image/')">
        <img
          :src="metadata.content.urls.download.url"
          class="overflow-hidden rounded-md"
        />
      </template>
      <template
        v-if="
          metadata.content.type.startsWith('audio/') ||
          metadata.content.type.startsWith('video/')
        "
      >
        <MediaPlayer :metadata="metadata" />
      </template>
    </CardContent>
  </Card>
</template>
