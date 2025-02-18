<script setup lang="ts">
import { Button } from '~/components/ui/button'
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '~/components/ui/card'
import { Label } from '~/components/ui/label'
import type {
  CollectionFragment,
  MetadataFragment,
} from '~/lib/graphql/graphql'
import { formatDateTime } from '~/lib/utils'
import { toast } from '~/components/ui/toast'

const props = defineProps<{
  content: CollectionFragment | MetadataFragment
}>()

const client = useBoscaClient()
const isLoading = ref(false)

async function setReady() {
  try {
    isLoading.value = true
    if (props.content.version) {
      await client.metadata.setReady(props.content.id)
    } else {
      await client.collections.setReady(props.content.id)
    }
    toast({
      title: 'Ready',
    })
  } catch (e) {
    console.error(e)
    toast({
      title: 'Failed to set content ready.',
    })
  } finally {
    isLoading.value = false
  }
}

async function onDownload() {
  const metadata = await client.metadata.get(props.content.id)
  document.location = metadata.content.urls.download.url
}
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>{{
        content.version ? 'Metadata Content' : 'Collection Ready'
      }}</CardTitle>
      <CardDescription>
        <span v-if="!content.ready">
          Before Bosca will process your {{
            content.version ? 'metadata' : 'collection'
          }} through workflows and publish, you must mark it as ready. Once you
          do this, you will be able to view the workflow progress and trigger
          new workflows, and you will be able to publish after it is
          transitioned to the Draft state.
          <br />
          <br />
          <b>NOTE:</b> This content is <b><i>NOT</i></b> ready. Click the button
          below to mark it ready.
        </span>
      </CardDescription>
    </CardHeader>
    <CardContent class="grid gap-6">
      <div class="flex items-center justify-between space-x-2">
        <Label for="ready" class="flex flex-col space-y-1">
          <span>Is Ready</span>
          <span class="font-normal leading-snug text-muted-foreground">
            <span v-if="!content.ready">
              When the {{ content.version ? 'metadata' : 'collection' }} is
              ready for the workflow engine to begin processing.
            </span>
            <span v-else>
              Your {{ content.version ? 'metadata' : 'collection' }} is ready
              for the workflow engine to begin processing.
            </span>
          </span>
        </Label>
        <div class="pe-1.5">
          <Button
            size="icon"
            v-if="!content.ready"
            @click="setReady"
            :disabled="isLoading"
          >
            <Icon name="i-lucide-check" class="size-4" />
          </Button>
          <Icon name="i-lucide-check" class="size-4 mr-2.5" v-else />
        </div>
      </div>
      <div v-if="content.ready">
        <Label for="ready" class="flex flex-col space-y-1">
          <span>Ready</span>
          <span class="font-normal leading-snug text-muted-foreground">
            {{ formatDateTime(content.ready) }}
          </span>
        </Label>
      </div>
      <div class="flex flex-col" v-if="content.version">
        <Label class="flex flex-col space-y-1">
          <span>Uploaded</span>
          <span class="font-normal leading-snug text-muted-foreground">
            <span v-if="content.uploaded">{{
              formatDateTime(content.uploaded)
            }}</span>
            <span v-else>--</span>
          </span>
        </Label>
      </div>
    </CardContent>
    <CardFooter v-if="content.version">
      <Button
        variant="outline"
        class="w-full"
        :disabled="!content.uploaded"
        @click="onDownload()"
      >
        Download
      </Button>
    </CardFooter>
  </Card>
</template>
