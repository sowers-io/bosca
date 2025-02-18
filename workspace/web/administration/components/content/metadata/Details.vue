<script setup lang="ts">
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '~/components/ui/card'
import type { MetadataFragment } from '~/lib/graphql/graphql'
import { Label } from '~/components/ui/label'
import { formatDateTime } from '~/lib/utils'

defineProps<{
  metadata: MetadataFragment
}>()
</script>
<template>
  <Card>
    <CardHeader>
      <CardTitle>Details</CardTitle>
      <CardDescription>{{ metadata.name }}</CardDescription>
    </CardHeader>
    <CardContent class="grid gap-6">
      <div class="flex flex-col">
        <Label class="flex flex-col space-y-1">
          <span>Content Type</span>
          <span class="font-normal leading-snug text-muted-foreground">
            {{ metadata.content.type }}
          </span>
        </Label>
      </div>
      <div class="flex flex-col">
        <Label class="flex flex-col space-y-1">
          <span>Language</span>
          <span class="font-normal leading-snug text-muted-foreground">
            {{ metadata.languageTag }}
          </span>
        </Label>
      </div>
      <div class="flex flex-col">
        <Label class="flex flex-col space-y-1">
          <span>Created</span>
          <span class="font-normal leading-snug text-muted-foreground">
            {{ formatDateTime(metadata.created) }}
          </span>
        </Label>
      </div>
      <div class="flex flex-col">
        <Label class="flex flex-col space-y-1">
          <span>Modified</span>
          <span class="font-normal leading-snug text-muted-foreground">
            {{ formatDateTime(metadata.modified) }}
          </span>
        </Label>
      </div>
    </CardContent>
  </Card>
</template>
