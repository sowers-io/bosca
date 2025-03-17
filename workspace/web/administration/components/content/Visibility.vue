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
import { Switch } from '~/components/ui/switch'
import type {
  CollectionFragment,
  MetadataFragment,
} from '~/lib/graphql/graphql'
import { toast } from '~/components/ui/toast'

const props = defineProps<{
  content: CollectionFragment | MetadataFragment
}>()

const client = useBoscaClient()

const isMetadata = computed(() => typeof props.content.version === 'number')
const isLoading = ref(false)
const isPublic = ref(props.content.public)
const isPublicList = ref(props.content.publicList)
const isContentPublic = ref(props.content.publicContent)
const isSupplementaryPublic = ref(props.content.publicSupplementary)

const isSaveEnabled = ref(false)

function checkIsSaveEnabled() {
  isSaveEnabled.value = isPublic.value != props.content.public ||
    isPublicList.value != props.content.publicList ||
    isContentPublic.value != props.content.publicContent ||
    isSupplementaryPublic.value != props.content.publicSupplementary
}

watch(isPublic, checkIsSaveEnabled)
watch(isPublicList, checkIsSaveEnabled)
watch(isContentPublic, checkIsSaveEnabled)
watch(isSupplementaryPublic, checkIsSaveEnabled)

async function save() {
  try {
    isLoading.value = true
    if (props.content.public != isPublic.value) {
      if (props.content.version) {
        await client.metadata.setPublic(props.content.id, isPublic.value)
      } else {
        await client.collections.setPublic(props.content.id, isPublic.value)
      }
    }
    if (props.content.isPublicList != isPublicList.value) {
      await client.collections.setPublicList(
        props.content.id,
        isPublicList.value,
      )
    }
    if (props.content.contentPublic != isContentPublic.value) {
      await client.metadata.setContentPublic(
        props.content.id,
        isContentPublic.value,
      )
    }
    if (props.content.publicSupplementary != isSupplementaryPublic.value) {
      await client.metadata.setSupplementaryPublic(
        props.content.id,
        isSupplementaryPublic.value,
      )
    }
    toast({
      title: 'Visibility Saved.',
    })
  } catch (e) {
    console.error(e)
    toast({
      title: 'Failed to save visibility.',
      description: (e as unknown as any).message,
    })
  } finally {
    setTimeout(() => {
      isLoading.value = false
    }, 1000)
  }
}

onUpdated(() => {
  isPublic.value = props.content.public
  isPublicList.value = props.content.publicList
  isContentPublic.value = props.content.publicContent
  isSupplementaryPublic.value = props.content.publicSupplementary
  checkIsSaveEnabled()
})
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Public Visibility</CardTitle>
      <CardDescription>
        Manage your {{ isMetadata ? 'Metadata' : 'Collection' }} visibility.
      </CardDescription>
    </CardHeader>
    <CardContent class="grid gap-6">
      <div class="flex items-center justify-between space-x-2">
        <Label for="public" class="flex flex-col space-y-1">
          <span>Public</span>
          <span class="font-normal leading-snug text-muted-foreground">
            Whether when published, the {{
              isMetadata ? 'Metadata' : 'Collection'
            }} will be publicly visible to anonymous users.
          </span>
        </Label>
        <Switch id="public" v-model:model-value="isPublic" />
      </div>
      <div
        class="flex items-center justify-between space-x-2"
        v-if="!isMetadata"
      >
        <Label for="publicListing" class="flex flex-col space-y-1">
          <span>Listing</span>
          <span class="font-normal leading-snug text-muted-foreground">
            Allow listing the contents of the Collection as an anonymous user.
          </span>
        </Label>
        <Switch id="publicListing" v-model:model-value="isPublicList" />
      </div>
      <div
        class="flex items-center justify-between space-x-2"
        v-if="isMetadata"
      >
        <Label for="publicContent" class="flex flex-col space-y-1">
          <span>Content</span>
          <span class="font-normal leading-snug text-muted-foreground">
            Whether when published, the Metadata content will be publicly
            available to anonymous users.
          </span>
        </Label>
        <Switch id="publicContent" v-model:model-value="isContentPublic" />
      </div>
      <div
        class="flex items-center justify-between space-x-2"
        v-if="isMetadata"
      >
        <Label for="publicSupplementary" class="flex flex-col space-y-1">
          <span>Supplementary</span>
          <span class="font-normal leading-snug text-muted-foreground">
            Whether when published, the Metadata supplementary content will be
            publicly available to anonymous users.
          </span>
        </Label>
        <Switch
          id="publicSupplementary"
          v-model:model-value="isSupplementaryPublic"
        />
      </div>
    </CardContent>
    <CardFooter>
      <Button
        variant="outline"
        class="w-full"
        :disabled="!isSaveEnabled || isLoading"
        @click="save"
      >
        Save Visibility
      </Button>
    </CardFooter>
  </Card>
</template>
