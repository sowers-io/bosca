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

const props = defineProps<{
  content: CollectionFragment | MetadataFragment
}>()

const router = useRouter()
const client = useBoscaClient()
const isLoading = ref(false)
const { data: traits } = client.workflows.getTraitsAsyncData()
const selectedTraitId = ref<string[]>(
  props.content.traitIds.map((t) => t) || [],
)
const hasChanges = ref(false)

function checkHasChanges() {
  hasChanges.value = JSON.stringify(props.content.traitIds.sort()) !==
    JSON.stringify(selectedTraitId.value.sort())
}

onUpdated(() => {
  selectedTraitId.value = props.content.traitIds.map((t) => t) || []
  checkHasChanges()
})

async function onSave() {
  isLoading.value = true
  try {
    const current = props.content.traitIds?.map((id) => id) || []
    for (const traitId of selectedTraitId.value) {
      if (
        !props.content.traitIds || !props.content.traitIds.includes(traitId)
      ) {
        await client.metadata.addTrait(props.content.id, traitId)
      } else if (props.content.traitIds) {
        current.splice(current.indexOf(traitId), 1)
      }
    }
    for (const traitId of current) {
      await client.metadata.removeTrait(props.content.id, traitId)
    }
    toast({ title: 'Traits saved.' })
  } catch (e: any) {
    toast({
      title: 'Failed to save traits.',
      description: e.message,
    })
  } finally {
    isLoading.value = false
  }
}

function onChecked(traitId: string) {
  if (selectedTraitId.value.includes(traitId)) {
    selectedTraitId.value = selectedTraitId.value.filter((id) => id !== traitId)
  } else {
    selectedTraitId.value.push(traitId)
  }
  checkHasChanges()
}
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Traits</CardTitle>
      <CardDescription>View the traits for this {{
          content.version ? 'Metadata' : 'Collection'
        }}.
      </CardDescription>
    </CardHeader>
    <CardContent>
      <Table>
        <TableHeader>
          <TableRow>
            <TableCell></TableCell>
            <TableCell>Name</TableCell>
            <TableCell>Description</TableCell>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow
            v-for="trait in traits"
            :key="trait.id"
          >
            <TableCell>
              <Checkbox
                :id="'trait-' + trait.id"
                :checked="selectedTraitId.includes(trait.id)"
                @update:checked="onChecked(trait.id)"
              />
            </TableCell>
            <TableCell>{{ trait.name }}</TableCell>
            <TableCell>{{ trait.description }}</TableCell>
          </TableRow>
        </TableBody>
      </Table>
      <div class="pt-4">
        <Button
          variant="outline"
          class="w-full"
          :disabled="!hasChanges || isLoading"
          @click="onSave"
        >
          Save Traits
        </Button>
      </div>
    </CardContent>
  </Card>
</template>
