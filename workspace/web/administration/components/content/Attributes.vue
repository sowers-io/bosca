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
import { toast } from '~/components/ui/toast'
import { Button } from '~/components/ui/button'
import JsonEditorVue from 'json-editor-vue'
import 'vanilla-jsoneditor/themes/jse-theme-dark.css'

const props = defineProps<{
  content: CollectionFragment | MetadataFragment
}>()

const client = useBoscaClient()
const isLoading = ref(false)
const attributes = ref(props.content.attributes)
const hasChanges = ref(false)

async function onSave() {
  isLoading.value = true
  try {
    let values = attributes.value
    if (typeof values === 'string') {
      values = JSON.parse(values)
    }
    if (props.content.version) {
      await client.metadata.setAttributes(props.content.id, values)
    } else {
      await client.collections.setAttributes(props.content.id, values)
    }
    toast({ title: 'Attributes saved.' })
  } catch (e: any) {
    toast({
      title: 'Failed to save attributes.',
      description: e.message,
    })
  } finally {
    isLoading.value = false
  }
}

watch(attributes, () => {
  hasChanges.value = true
})

</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Attributes</CardTitle>
      <CardDescription>View the attributes for this {{
          content.version ? 'Metadata' : 'Collection'
        }}.
      </CardDescription>
    </CardHeader>
    <CardContent>
      <JsonEditorVue
          class="jse-theme-dark"
          v-model="attributes"
      />
      <div class="pt-4">
        <Button
          variant="outline"
          class="w-full"
          :disabled="!hasChanges || isLoading"
          @click="onSave"
        >
          Save Attributes
        </Button>
      </div>
    </CardContent>
  </Card>
</template>

<style>
.jse-menu {
  background-color: #222222 !important;
  @apply bg-background mb-4;
}
</style>
