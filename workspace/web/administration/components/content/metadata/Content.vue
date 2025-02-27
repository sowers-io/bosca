<script setup lang="ts">
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '~/components/ui/card'
import type {MetadataFragment} from '~/lib/graphql/graphql.ts'
import EditorViewOnly from "~/components/content/metadata/editor/EditorViewOnly.vue";

const props = defineProps<{
  metadata: MetadataFragment
}>()

const client = useBoscaClient()
const document = props.metadata.content.type === 'bosca/v-document' ? await client.metadata.getDocument(props.metadata.id) : null

const textContent = asyncComputed(async () => {
  if (props.metadata.content.type.startsWith('text/')) {
    return await $fetch(props.metadata.content.urls.download.url)
  }
  return null
})

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
        <MediaPlayer :metadata="metadata"/>
      </template>
      <template v-if="document">
        <EditorViewOnly :metadata="metadata" :document="document"/>
      </template>
      <template v-if="textContent">
        {{ textContent }}
      </template>
    </CardContent>
  </Card>
</template>
