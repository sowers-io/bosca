<script setup lang="ts">

import type {CollectionFragment, CollectionIdNameFragment} from "~/lib/graphql/graphql.ts";

const props = defineProps<{
  collection: string | CollectionFragment | CollectionIdNameFragment | null
}>()

const client = useBoscaClient()

const collection = asyncComputed(async () => {
  if (typeof props.collection === 'string') {
    return await client.collections.get(props.collection)
  }
  return props.collection
})

const imageId = asyncComputed(async () => {
  if (!props.collection) return null
  const id = typeof props.collection === 'string' ? props.collection : props.collection?.id
  if (!id) return null
  const relationships = await client.collections.getRelationships(id)
  return relationships?.find(r => r.relationship === 'avatar' || r.relationship === 'image.featured')?.metadata?.id
}, null)
</script>
<template>
  <div class="flex items-center">
    <img v-if="imageId" :src="'/content/file?id=' + imageId" :alt="collection?.name" class="w-6 h-6 bg-background mr-3 overflow-hidden rounded-full">
    {{ collection?.name }}
  </div>
</template>