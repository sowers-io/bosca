<script setup lang="ts">
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '~/components/ui/card'
import type {
  CollectionFragment,
  MetadataFragment,
} from '~/lib/graphql/graphql'
import { Label } from '~/components/ui/label'
import { formatDateTime } from '~/lib/utils'
import type { CollectionItem } from '~/lib/bosca/contentcollection'

defineProps<{
  collection: CollectionFragment
  items: CollectionItem[]
}>()
</script>
<template>
  <Card>
    <CardHeader>
      <CardTitle>Items</CardTitle>
    </CardHeader>
    <CardContent class="grid gap-6">
      <ContentCollectionsList :collection="collection" :items="items" />
    </CardContent>
  </Card>
</template>
