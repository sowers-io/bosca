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

defineProps<{
  collection: CollectionFragment
}>()
</script>
<template>
  <Card>
    <CardHeader>
      <CardTitle>Details</CardTitle>
      <CardDescription>{{ collection.name }}</CardDescription>
    </CardHeader>
    <CardContent class="grid gap-6">
      <div class="flex flex-col">
        <Label class="flex flex-col space-y-1">
          <span>Slug</span>
          <span class="font-normal leading-snug text-muted-foreground">
            {{ collection.slug || '--' }}
          </span>
        </Label>
      </div>
      <div class="flex flex-col">
        <Label class="flex flex-col space-y-1">
          <span>Created</span>
          <span class="font-normal leading-snug text-muted-foreground">
            {{ formatDateTime(collection.created) }}
          </span>
        </Label>
      </div>
      <div class="flex flex-col">
        <Label class="flex flex-col space-y-1">
          <span>Modified</span>
          <span class="font-normal leading-snug text-muted-foreground">
            {{ formatDateTime(collection.modified) }}
          </span>
        </Label>
      </div>
    </CardContent>
  </Card>
</template>
