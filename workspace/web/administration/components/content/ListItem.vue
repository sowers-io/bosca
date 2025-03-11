<script setup lang="ts">
import type {
  CollectionFragment,
  CollectionIdNameFragment,
  MetadataFragment,
  MetadataIdNameFragment,
  ProfileFragment,
  ProfileIdNameFragment,
} from '~/lib/graphql/graphql.ts'

const props = defineProps<{
  item:
    | CollectionFragment
    | CollectionIdNameFragment
    | MetadataFragment
    | MetadataIdNameFragment
    | ProfileFragment
    | ProfileIdNameFragment
}>()

const client = useBoscaClient()

const imageRelationship = asyncComputed(async () => {
  if (!props.item?.id) return null
  if (props.item.__typename === 'Profile') {
    // TODO
    return null
  }
  const relationships = props.item.__typename === 'Collection'
    ? await client.collections.getMetadataRelationships(props.item.id)
    : await client.metadata.getRelationships(props.item.id!)
  return relationships?.find((r) =>
    r.relationship === 'image.avatar' || r.relationship === 'image.featured'
  )
}, null)

const imageId = computed(() => {
  return imageRelationship.value?.metadata?.id
})
</script>
<template>
  <div class="flex items-center">
    <Icon
      name="i-lucide-folder"
      class="size-4 m-2 mr-5"
      v-if="!imageId && item.__typename === 'Collection'"
    />
    <Icon
      name="i-lucide-file"
      class="size-4 m-2 mr-5"
      v-if="!imageId && item.__typename === 'Metadata'"
    />
    <Icon
      name="i-lucide-user"
      class="size-4 m-2 mr-5"
      v-if="!imageId && item.__typename === 'Profile'"
    />
    <img
      v-if="imageId"
      :src="'/content/file?id=' + imageId"
      :alt="item?.name || ''"
      :class="
        'w-8 h-8 bg-background mr-3 overflow-hidden object-cover' +
        (imageRelationship?.relationship === 'image.avatar'
          ? ' rounded-full'
          : ' rounded-md')
      "
    >
    {{ item?.name }}
  </div>
</template>
