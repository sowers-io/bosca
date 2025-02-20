<script lang="ts" setup>
import {BubbleMenu, EditorContent, type Range, useEditor} from '@tiptap/vue-3'
import Document from '@tiptap/extension-document'
import Placeholder from '@tiptap/extension-placeholder'
import StarterKit from '@tiptap/starter-kit'
import Commands from '@/lib/editor/commands'
import Suggestion from '@/lib/editor/suggestion'
import {
  type CollectionIdNameFragment, type CollectionParentsFragment, DocumentAttributeType,
  DocumentAttributeUiType,
  type DocumentFragment,
  type DocumentInput,
  type DocumentTemplateAttribute,
  type DocumentTemplateFragment,
  type MetadataFragment,
  type MetadataProfile,
  type MetadataProfileFragment,
  type MetadataProfileInput,
  type MetadataRelationshipFragment, type ParentCollectionFragment,
  ProfileVisibility,
} from '@/lib/graphql/graphql'
import {toast} from '~/components/ui/toast'
import {Uploader} from '@/lib/uploader'
import {Image} from '@/lib/editor/image'
import {hideAll} from 'tippy.js'
import {toMetadataInput} from '@/lib/metadata'
import {CommandItems, OpenMediaPickerEvent} from '@/lib/editor/commanditems'
import {Plugin} from '@tiptap/pm/state'
import type {ContentTypeFilter} from '~/lib/bosca/contentmetadata'

const client = useBoscaClient()
const uploader = new Uploader(client)

const mediaDialogOpen = ref(false)

const props = defineProps<{
  metadata: MetadataFragment
  documentCollection: ParentCollectionFragment | null | undefined
  parents: CollectionIdNameFragment[] | null | undefined
  document: DocumentFragment
  template: DocumentTemplateFragment | null | undefined
}>()

const filter = ref<ContentTypeFilter>({
  jpg: true,
  png: true,
  webp: true,
  mp4: true,
  mp3: true,
  webm: true,
})

const metadata = defineModel('metadata', {type: Object, default: null})

const relationships: Ref<Array<MetadataRelationshipFragment>> = ref([])

const update = ref(0)
const title = ref(props.metadata.name)

let loading: { [key: string]: boolean } = {}

let inputAttributes: { [key: string]: any } = {}
let inputOverrideAttributes: { [key: string]: any } = {}

let collectionAttributes: { [key: string]: CollectionIdNameFragment[] } = {}
let collectionOverrideAttributes: { [key: string]: CollectionIdNameFragment[] } = {}

let profileAttributes: {
  [key: string]: MetadataProfileFragment | undefined | null
} = {}
let relationshipAttributes: {
  [key: string]: MetadataRelationshipFragment | undefined | null
} = {}

async function updateAttributes() {
  if (editor.value) {
    const editable = props.metadata.workflow.state === 'draft'
    if (editable !== editor.value.isEditable) {
      editor.value.setEditable(editable, true)
    }
  }
  inputAttributes = {}
  profileAttributes = {}
  collectionAttributes = {}
  relationshipAttributes = {}
  for (const attr of props.template?.attributes || []) {
    switch (attr.ui) {
      case DocumentAttributeUiType.Textarea:
      case DocumentAttributeUiType.Input:
        inputAttributes[attr.key] = inputOverrideAttributes[attr.key] ||
            props.metadata.attributes[attr.key]
        break
      case DocumentAttributeUiType.Profile:
        profileAttributes[attr.key] = props.metadata.profiles.find((p) =>
            p.relationship === attr.key
        ) as MetadataProfile | undefined | null
        break
      case DocumentAttributeUiType.Collection:
        collectionAttributes[attr.key] = collectionOverrideAttributes[attr.key] || props.parents || []
        break
      case DocumentAttributeUiType.Image:
      case DocumentAttributeUiType.File:
        relationshipAttributes[attr.key] = relationships.value.find((r) =>
            r.relationship === attr.key
        )
        break
    }
  }
  update.value++
}

const CustomDocument = Document.extend({
  content: props.template?.configuration?.content || 'heading block+',
  addProseMirrorPlugins() {
    return [
      new Plugin({
        appendTransaction: (transactions, oldState, newState) => {
          const {doc, tr} = newState
          let h1Count = 0
          doc.descendants((node, pos) => {
            if (node.type.name === 'heading' && node.attrs.level === 1) {
              h1Count++
              if (h1Count > 1) {
                tr.setNodeAttribute(pos, 'level', 2)
              }
            }
          })
          return h1Count > 1 ? tr : null
        },
      }),
    ]
  },
})

const editor = useEditor({
  content: props.document.content.document,
  editable: props.metadata.workflow.state === 'draft',
  extensions: [
    CustomDocument,
    StarterKit.configure({document: false}),
    Image,
    Placeholder.configure({
      showOnlyCurrent: false,
      placeholder: ({node}) => {
        if (
            // @ts-ignore
            node.type.name === 'heading' && node.attrs.level === 1
        ) {
          return 'Title...'
        }
        return 'Type / to view options'
      },
    }),
    Commands.configure({
      suggestion: Suggestion,
    }),
  ],
  onUpdate: ({editor}) => {
    const node = editor.view.dom.childNodes[0]
    title.value = node ? (node as HTMLElement)?.innerText : ''
  },
  editorProps: {
    attributes: {
      class:
          'flex flex-col focus:outline-none w-full h-full prose prose-sm prose-p:my-1',
    },
    handleDrop(view, event) {
      event.preventDefault()
      const fileList = event.dataTransfer?.files
      if (!fileList?.length) return false
      const files: File[] = []
      for (let i = 0; i < fileList.length; i++) {
        const file = fileList.item(i)
        if (file) {
          files.push(file)
        }
      }

      async function doUpload() {
        toast({title: 'Uploading files, please wait...'})
        try {
          const metadataIds = await uploader.upload(files)
          const coords = view.posAtCoords({
            left: event.clientX,
            top: event.clientY,
          })
          if (coords) {
            for (const metadataId of metadataIds) {
              const image = view.state.schema.nodes.image.create({
                src: '/content/image?id=' + metadataId,
                metadataId: metadataId,
              })
              view.dispatch(view.state.tr.insert(coords.pos, image))
            }
          }
          toast({title: 'File(s) uploaded'})
        } catch (e) {
          toast({
            title: 'Error uploading file(s)',
            description: (e as unknown as any).message,
          })
        }
      }

      doUpload()
      return true
    },
  },
})

enum MediaPickerType {
  EMBED,
  ATTRIBUTE,
}

let pendingRange: Range | null | undefined = null
let mediaPickerType: MediaPickerType = MediaPickerType.EMBED
let mediaPickerAttribute: DocumentTemplateAttribute | null = null

async function onMediaPicked(id: string) {
  try {
    switch (mediaPickerType) {
      case MediaPickerType.ATTRIBUTE:
        const metadata = await client.metadata.get(id)
        onMetadataAttributeChanged(mediaPickerAttribute!, metadata)
        break
      case MediaPickerType.EMBED:
        await onAddImage(id)
        break
    }
  } finally {
    mediaPickerType = MediaPickerType.EMBED
    mediaDialogOpen.value = false
  }
}

function onTextAttributeChanged(
    attribute: DocumentTemplateAttribute,
    value: string,
) {
  delete inputOverrideAttributes[attribute.key]
  inputAttributes[attribute.key] = value
}

function onCollectionAttributeChanged(
    attribute: DocumentTemplateAttribute,
    collections: CollectionIdNameFragment[] | null,
) {
  delete collectionOverrideAttributes[attribute.key]
  collectionAttributes[attribute.key] = collections || []
  update.value++
}

function onClickMetadataAttribute(attribute: DocumentTemplateAttribute) {
  hideAll()

  if (attribute.ui === DocumentAttributeUiType.Image) {
    filter.value = {
      jpg: true,
      png: true,
      webp: true,
      mp4: false,
      mp3: false,
      webm: false,
    }
  } else {
    filter.value = {
      jpg: false,
      png: false,
      webp: false,
      mp4: true,
      mp3: true,
      webm: true,
    }
  }

  pendingRange = null
  mediaPickerType = MediaPickerType.ATTRIBUTE
  mediaPickerAttribute = attribute
  mediaDialogOpen.value = true
}

function onMetadataAttributeChanged(
    attribute: DocumentTemplateAttribute,
    metadata: MetadataFragment | null,
) {
  if (!metadata) {
    relationshipAttributes[attribute.key] = null
  } else {
    relationshipAttributes[attribute.key] = {
      metadata: metadata!,
      attributes: {},
      relationship: attribute.key,
    }
  }
  update.value++
}

async function onRunWorkflow(attribute: DocumentTemplateAttribute) {
  if (loading[attribute.key]) return
  hideAll()
  let workflows = ''
  switch (attribute.ui) {
    case DocumentAttributeUiType.Textarea:
    case DocumentAttributeUiType.Input:
      inputOverrideAttributes[attribute.key] = ''
      inputAttributes[attribute.key] = ''
      break;
    case DocumentAttributeUiType.Collection:
      collectionOverrideAttributes[attribute.key] = []
      collectionAttributes[attribute.key] = []
      break;
  }
  for (const workflow of attribute.workflows) {
    await client.workflows.enqueueMetadataWorkflow(
        workflow.workflow!.id,
        props.metadata.id,
        props.metadata.version,
    )
    if (workflows.length > 0) workflows += ', '
    workflows += workflow.workflow!.name
  }
  loading[attribute.key] = true
  await updateAttributes()
  toast({title: 'Executing: ' + attribute.description})
}

function onClickProfileAttribute(attribute: DocumentTemplateAttribute) {
  hideAll()
}

async function onProfileAttributeChanged(
    attribute: DocumentTemplateAttribute,
    profile: MetadataProfileInput | null,
) {
  if (!profile?.profileId) {
    profileAttributes[attribute.key] = null
    update.value++
    return
  }
  profileAttributes[attribute.key] = {
    profile: await client.profiles.getProfile(profile.profileId),
    relationship: profile?.relationship,
  }
  update.value++
}

async function onAddImage(id: string) {
  const e = editor.value
  if (!e) return
  let chain = e.chain().focus()
  if (pendingRange) {
    chain = chain.deleteRange(pendingRange)
    pendingRange = null
  }
  chain
      .setImage({
        src: '/content/image?id=' + id,
        metadataId: id,
      })
      .setTextSelection(e.state.selection.to + 1)
      .insertContent({type: 'paragraph'})
      .run()
}

function onOpenMediaPicker(event: OpenMediaPickerEvent) {
  hideAll()
  pendingRange = event.range
  mediaDialogOpen.value = true
}

async function onSave() {
  const e = editor.value
  if (!e) return
  try {
    const newDocument: DocumentInput = {
      templateMetadataId: props.document.templateMetadataId,
      templateMetadataVersion: props.document.templateMetadataVersion,
      title: title.value,
      content: {
        document: e.state.doc.toJSON(),
      },
    }
    const input = toMetadataInput(props.metadata)
    input.document = newDocument
    input.name = title.value
    for (const attr of props.template?.attributes || []) {
      if (
          attr.ui === DocumentAttributeUiType.Input ||
          attr.ui === DocumentAttributeUiType.Textarea
      ) {
        if (!input.attributes) input.attributes = {}
        input.attributes[attr.key] = inputAttributes[attr.key]
      } else if (attr.ui === DocumentAttributeUiType.Profile) {
        if (!input.profiles) input.profiles = []
        const profile = profileAttributes[attr.key]
        if (!profile?.profile?.id) continue
        input.profiles.push({
          profileId: profile.profile!.id!,
          relationship: attr.key,
        })
      }
    }
    await client.metadata.edit(props.metadata.id, input)

    inputOverrideAttributes = {}
    collectionOverrideAttributes = {}

    for (const collection of props.parents || []) {
      await client.collections.removeMetadata(collection.id, props.metadata.id)
    }

    for (const key in collectionAttributes) {
      const collections = collectionAttributes[key]
      if (!collections) continue
      for (const collection of collections) {
        await client.collections.addMetadata(collection.id, props.metadata.id)
      }
    }

    for (const attr of props.template?.attributes || []) {
      if (
          attr.ui === DocumentAttributeUiType.Image ||
          attr.ui === DocumentAttributeUiType.File
      ) {
        const removeRelationshipId = relationships.value.find((r) =>
            r.relationship === attr.key
        )?.metadata?.id
        if (removeRelationshipId) {
          await client.metadata.removeRelationship(
              props.metadata.id,
              removeRelationshipId,
              attr.key,
          )
        }
        const relationship = relationshipAttributes[attr.key]
        if (!relationship) continue
        await client.metadata.addRelationship({
          id1: props.metadata.id,
          id2: relationship.metadata.id,
          attributes: {},
          relationship: attr.key,
        })
      }
    }
  } catch (e) {
    console.error('Error saving document', e)
    toast({
      title: 'Error saving document',
      description: (e as unknown as any).message,
    })
  }
}

onMounted(async () => {
  relationships.value = await client.metadata.getRelationships(
      props.metadata.id,
  )
  await updateAttributes()
  window.addEventListener('save-document', onSave)
  // @ts-ignore
  window.addEventListener(OpenMediaPickerEvent.NAME, onOpenMediaPicker)
})

onUnmounted(() => {
  window.removeEventListener('save-document', onSave)
  // @ts-ignore
  window.removeEventListener(OpenMediaPickerEvent.NAME, onOpenMediaPicker)
})

watch(metadata, async () => {
  relationships.value = await client.metadata.getRelationships(
      props.metadata.id,
  )
  await updateAttributes()
})

client.listeners.onMetadataSupplementaryChanged(async (id, key) => {
  const m = metadata.value as MetadataFragment
  if (id === m?.id) {
    for (const attr of props.template?.attributes || []) {
      if (attr.supplementaryKey !== key) continue
      switch (attr.ui) {
        case DocumentAttributeUiType.Textarea:
        case DocumentAttributeUiType.Input: {
          inputOverrideAttributes[attr.key] = await client.metadata.getSupplementaryText(id, key)
          console.log(inputOverrideAttributes)
          if (inputOverrideAttributes[attr.key]) {
            loading[attr.key] = false
            toast({title: 'Finished: ' + attr.description})
          }
          inputAttributes[attr.key] = inputOverrideAttributes[attr.key]
          await updateAttributes()
          break;
        }
        case DocumentAttributeUiType.Collection: {
          const collections = await client.metadata.getSupplementaryJson(id, key)
          collectionOverrideAttributes[attr.key] = collections.collections
          if (collectionOverrideAttributes[attr.key]) {
            loading[attr.key] = false
            toast({title: 'Finished: ' + attr.description})
          }
          collectionAttributes[attr.key] = collectionOverrideAttributes[attr.key]
          await updateAttributes()
          break;
        }
        default:
          loading[attr.key] = false
          update.value++
          toast({title: 'Finished: ' + attr.description})
          break;
      }
      break
    }
  }
})
</script>

<template>
  <div class="w-full h-full" v-if="editor" :data-update="update">
    <bubble-menu
        class="flex border bg-background gap-1 rounded-md p-1 drop-shadow-xl ms-2"
        :tippy-options="{ duration: 100, offset: [0, 20] }"
        :editor="editor"
    >
      <div class="flex items-center space-x-2">
        <button
            v-for="(item, index) in CommandItems"
            :key="index.toString() + '-' + item.name"
            :class="
            {
              'inline-flex size-8 rounded-md': true,
              'items-center justify-center hover:bg-accent hover:text-accent-foreground':
                true,
              'bg-accent text-foreground': item.name &&
                editor.isActive(item.name, item.attributes),
            }
          "
            @click="item.command({ editor })"
        >
          <Icon :name="item.icon" class="h-4 w-4"/>
        </button>
      </div>
    </bubble-menu>

    <div class="grid grid-cols-3 gap-2 h-full w-full">
      <editor-content class="w-full h-full col-span-2" :editor="editor"/>
      <div class="min-h-[calc(100dvh-170px)]">
        <div class="bg-accent rounded-md px-4 py-2 h-full">
          <div v-for="attr in template?.attributes || []" :key="attr.key">
            <template v-if="attr.ui === DocumentAttributeUiType.Collection">
              <ContentMetadataEditorCollectionAttribute
                  :attribute="attr as DocumentTemplateAttribute"
                  :editable="metadata.workflow.state === 'draft'"
                  :collections="collectionAttributes[attr.key] || []"
                  :loading="loading[attr.key]"
                  :on-change="onCollectionAttributeChanged"
                  :on-run-workflow="onRunWorkflow"
              />
            </template>
            <template v-if="attr.ui === DocumentAttributeUiType.Input">
              <ContentMetadataEditorInputAttribute
                  :attribute="attr as DocumentTemplateAttribute"
                  :editable="metadata.workflow.state === 'draft'"
                  :value="inputAttributes[attr.key]"
                  :loading="loading[attr.key]"
                  :on-change="onTextAttributeChanged"
                  :on-run-workflow="onRunWorkflow"
              />
            </template>
            <template v-if="attr.ui === DocumentAttributeUiType.Textarea">
              <ContentMetadataEditorTextAreaAttribute
                  :attribute="attr as DocumentTemplateAttribute"
                  :editable="metadata.workflow.state === 'draft'"
                  :value="inputAttributes[attr.key]"
                  :loading="loading[attr.key]"
                  :on-change="onTextAttributeChanged"
                  :on-run-workflow="onRunWorkflow"
              />
            </template>
            <template v-if="attr.ui === DocumentAttributeUiType.Image">
              <ContentMetadataEditorImageAttribute
                  :attribute="attr as DocumentTemplateAttribute"
                  :editable="metadata.workflow.state === 'draft'"
                  :metadata="
                  relationshipAttributes[attr.key]?.metadata as
                  | MetadataFragment
                  | null
                "
                  :uploader="uploader"
                  :on-click="onClickMetadataAttribute"
                  :on-change="onMetadataAttributeChanged"
              />
            </template>
            <template v-if="attr.ui === DocumentAttributeUiType.File">
              <ContentMetadataEditorFileAttribute
                  :attribute="attr as DocumentTemplateAttribute"
                  :editable="metadata.workflow.state === 'draft'"
                  :relationship="relationshipAttributes[attr.key]"
                  :uploader="uploader"
                  :on-click="onClickMetadataAttribute"
                  :on-change="onMetadataAttributeChanged"
              />
            </template>
            <template v-if="attr.ui === DocumentAttributeUiType.Profile">
              <ContentMetadataEditorProfileAttribute
                  :attribute="attr as DocumentTemplateAttribute"
                  :editable="metadata.workflow.state === 'draft'"
                  :profile="
                  profileAttributes[attr.key] as
                  | MetadataProfile
                  | null
                "
                  :on-click="onClickProfileAttribute"
                  :on-change="onProfileAttributeChanged"
              />
            </template>
          </div>
        </div>
      </div>
    </div>

    <Dialog v-model:open="mediaDialogOpen">
      <DialogContent
          class="h-[calc(100dvh-100px)] w-[calc(100dvw-100px)] max-w-full overflow-y-auto"
      >
        <div class="flex flex-col gap-2 h-full">
          <h1 class="font-bold">Click to Select Your Item</h1>
          <ContentMedia :filter="filter" :on-selected="onMediaPicked"/>
        </div>
      </DialogContent>
    </Dialog>
  </div>
</template>

<style>
.tiptap {
  .is-empty:first-child::before, .is-empty:last-child::before {
    @apply float-left h-0 text-gray-300 dark:text-gray-800 pointer-events-none;
    content: attr(data-placeholder);
  }
}
</style>
