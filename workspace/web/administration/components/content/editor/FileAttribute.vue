<script lang="ts" setup>
import { toast } from '~/components/ui/toast'
import type { AttributeState } from '~/lib/attribute.ts'
import type { MetadataRelationshipFragment } from '~/lib/graphql/graphql.ts'

const props = defineProps<{
  attribute: AttributeState | null | undefined
  uploader: Uploader
  editable: boolean
}>()

const client = useBoscaClient()
const dropzone = ref()
const dialogOpen = ref(false)

const metadata = asyncComputed(async () => {
  if (props.attribute?.value) {
    const relationship = props.attribute.value as MetadataRelationshipFragment
    return await client.metadata.get(relationship.metadata.id)
  }
  return null
})

useDropZone(dropzone, {
  onDrop: async function (files: File[] | null) {
    if (!props.editable) return
    if (!files || files.length === 0) return
    toast({ title: 'Uploading files, please wait...' })
    try {
      const metadataIds = await props.uploader.upload(files)
      console.log(metadataIds)
      await onMetadataSelected(metadataIds[0])
      toast({ title: 'File(s) uploaded' })
    } catch (e) {
      console.error('Error uploading file(s)', e)
      toast({
        title: 'Error uploading file(s)',
        description: (e as unknown as any).message,
      })
    }
  },
  multiple: false,
  preventDefaultForUnhandled: false,
})

async function onMetadataSelected(id: string) {
  if (!props.attribute) return
  dialogOpen.value = false
  const metadata = await client.metadata.get(id)
  props.attribute.value = {
    metadata: {
      id: metadata.id,
      name: metadata.name,
    },
    relationship: props.attribute.configuration.relationship,
  } as MetadataRelationshipFragment
}
</script>
<template>
  <div v-if="attribute">
    <label class="block font-bold mt-4 mb-2">{{ attribute.name }}</label>
    <div
      v-if="editable && !attribute.value"
      ref="dropzone"
      class="cursor-pointer overflow-hidden bg-background rounded-md shadow-sm"
      @click="dialogOpen = true"
    >
      <div
        class="flex w-full h-32 justify-center items-center text-gray-200 text-xl font-bold"
      >
        Click or Drop File Here
      </div>
    </div>
    <div v-else-if="metadata">
      <template
        v-if="
          metadata.content.type.startsWith('audio/') ||
          metadata.content.type.startsWith('video/') ||
          metadata.content.type == 'bosca/x-youtube-video'
        "
      >
        <MediaPlayer :metadata="metadata" />
      </template>
      <div class="grid justify-items-end" v-if="editable && attribute.value">
        <Button variant="ghost" @click="attribute.value = null">
          Clear
        </Button>
      </div>
    </div>
    <div v-else>
      No File
    </div>

    <Dialog v-model:open="dialogOpen">
      <DialogContent
        class="h-[calc(100dvh-100px)] w-[calc(100dvw-100px)] max-w-full overflow-y-auto"
      >
        <div class="flex flex-col gap-2 h-full">
          <h1 class="font-bold">Click to Select Your Item</h1>
          <ContentMedia
            :filter="
              {
                mp4: true,
                webm: true,
                mp3: true,
                jpg: false,
                png: false,
                webp: false,
                youtube: true,
              }
            "
            :on-selected="onMetadataSelected"
          />
        </div>
      </DialogContent>
    </Dialog>
  </div>
</template>
