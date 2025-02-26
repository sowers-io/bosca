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

const imageRelationship = asyncComputed(async () => {
  if (!props.collection) return null
  const id = typeof props.collection === 'string' ? props.collection : props.collection?.id
  if (!id) return null
  const relationships = await client.collections.getRelationships(id)
  return relationships?.find(r => r.relationship === 'image.avatar' || r.relationship === 'image.featured')
}, null)
const imageId = computed(() => {
  return imageRelationship.value?.metadata?.id
})
</script>
<template>
  <div class="flex items-center">
    <img v-if="imageId" :src="'/content/file?id=' + imageId" :alt="collection?.name" :class="'w-8 h-8 bg-background mr-3 overflow-hidden' + (imageRelationship?.relationship === 'image.avatar' ? ' rounded-full' : ' rounded-md')">
    {{ collection?.name }}
  </div>
</template>