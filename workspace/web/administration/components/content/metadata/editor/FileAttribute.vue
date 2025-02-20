<script lang="ts" setup>
import type {
  DocumentTemplateAttribute,
  MetadataFragment, MetadataRelationship, MetadataRelationshipFragment,
} from '~/lib/graphql/graphql'
import {toast} from '~/components/ui/toast'

const props = defineProps<{
  attribute: DocumentTemplateAttribute
  relationship?: MetadataRelationshipFragment | null | undefined
  uploader: Uploader
  editable: boolean
  onChange: (attribute: DocumentTemplateAttribute, value: any) => void
  onClick: (attribute: DocumentTemplateAttribute) => void
}>()

const client = useBoscaClient()
const dropzone = ref()

const metadata = asyncComputed(async () => {
  if (props.relationship?.metadata?.id) {
    return await client.metadata.get(props.relationship.metadata.id)
  }
  return null
})

useDropZone(dropzone, {
  onDrop: async function (files: File[] | null) {
    if (!props.editable) return
    if (!files || files.length === 0) return
    toast({title: 'Uploading files, please wait...'})
    try {
      const metadataIds = await props.uploader.upload(files)
      const metadata = await client.metadata.get(metadataIds[0])
      props.onChange(props.attribute, metadata)
      toast({title: 'File(s) uploaded'})
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
</script>
<template>
  <div>
    <label class="block font-bold mt-4 mb-2">{{ attribute.name }}</label>
    <div
        v-if="editable && !relationship"
        ref="dropzone"
        class="cursor-pointer overflow-hidden bg-background rounded-md"
        @click="onClick(attribute)"
    >
      <div
          class="flex w-full h-32 justify-center items-center text-gray-200 text-xl font-bold"
      >
        Click or Drop File Here
      </div>
    </div>
    <div v-else-if="metadata">
      <template v-if="metadata.content.type.startsWith('audio/') || metadata.content.type.startsWith('video/')">
        <MediaPlayer :metadata="metadata"/>
      </template>
      <div class="grid justify-items-end" v-if="editable && relationship">
        <Button variant="ghost" @click="onChange(attribute, null)">
          Clear
        </Button>
      </div>
    </div>
    <div v-else>
      No File
    </div>
  </div>
</template>
