<script setup lang="ts">
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '~/components/ui/card'
import type {
  CollectionFragment,
  MetadataFragment,
} from '~/lib/graphql/graphql'
import { toast } from '~/components/ui/toast'
import { Button } from '~/components/ui/button'
import { Label } from '~/components/ui/label'

const props = defineProps<{
  content: CollectionFragment | MetadataFragment
}>()

const client = useBoscaClient()
const { data: states } = client.workflows.getStatesAsyncData()
const stateId = ref(props.content.workflow.state)
const isLoading = ref(false)
const isPending = ref(false)
const pending = computed(() => {
  if (props.content.workflow.pending) {
    return states.value?.find((s) => s.id === props.content.workflow.pending)
      ?.name
  }
  return null
})

function updatePending() {
  isPending.value = stateId.value != props.content.workflow.state
}

watch(stateId, updatePending)
onUpdated(() => {
  stateId.value = props.content.workflow.state
  updatePending()
})

async function onSave() {
  isLoading.value = true
  try {
    if (props.content.version) {
      await client.metadata.beginTransition(
        props.content.id,
        props.content.version,
        stateId.value,
        'User Requested Change',
      )
    } else {
      await client.collections.beginTransition(
        props.content.id,
        stateId.value,
        'User Requested Change',
      )
    }
    toast({
      title: 'State Transition Requested.',
    })
  } catch (e) {
    stateId.value = props.content.workflow.state
    updatePending()
    console.error(e)
    toast({
      title: 'Failed to save state.',
      description: (e as unknown as any).message,
    })
  } finally {
    setTimeout(() => {
      isLoading.value = false
    }, 1000)
  }
}
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>States</CardTitle>
      <CardDescription>Manage the current state for the {{
          content.version ? 'Metadata' : 'Collection'
        }}.
      </CardDescription>
    </CardHeader>
    <CardContent>
      <div>
        <Label for="state">State</Label>
        <Select id="state" v-model="stateId" class="w-full">
          <SelectTrigger class="w-full">
            <SelectValue placeholder="Select a Workflow State" />
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              <SelectLabel>States</SelectLabel>
              <SelectItem
                v-for="state in states"
                :value="state.id"
                :key="state.id"
              >
                {{ state.name }}
              </SelectItem>
            </SelectGroup>
          </SelectContent>
        </Select>
      </div>
      <div class="mt-4" v-if="pending">
        <Label class="flex flex-col space-y-1">
          <span>Pending State</span>
          <span class="font-normal leading-snug text-muted-foreground">
            {{ pending }}
          </span>
        </Label>
      </div>
    </CardContent>
    <CardFooter>
      <Button
        variant="outline"
        class="w-full"
        @click="onSave"
        :disabled="!isPending || isLoading || content.workflow.pending"
      >Save State</Button>
    </CardFooter>
  </Card>
</template>
