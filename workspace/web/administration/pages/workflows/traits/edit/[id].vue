<script setup lang="ts">
import { toast } from '~/components/ui/toast'

const client = useBoscaClient()
const route = useRoute()
const { data: trait } = client.workflows.getTraitAsyncData(
  route.params.id.toString(),
)
const { data: workflows } = client.workflows.getAllAsyncData()

const name = ref('')
const description = ref('')
const contentTypes = ref<string[]>([])
const deleteId = ref<string | undefined>(undefined)
const workflowIds = ref<string[]>([])
const isLoading = ref(false)

function onUpdate() {
  const t = trait.value
  if (!t) return
  name.value = t.name
  description.value = t.description
  deleteId.value = t.deleteWorkflowId || undefined
  contentTypes.value = t.contentTypes
  workflowIds.value = t.workflowIds
}

watch(trait, onUpdate)

function onSubmit(e: Event) {
  e.preventDefault()
  e.stopImmediatePropagation()
  isLoading.value = true
  try {
    client.workflows.editTrait({
      contentTypes: contentTypes.value,
      deleteWorkflowId: deleteId.value,
      description: description.value,
      id: trait.value!.id,
      name: name.value,
      workflowIds: workflowIds.value,
    })
    navigateTo('/workflows/traits')
  } catch (e: any) {
    toast({
      title: 'Failed to save.',
      description: e.message,
    })
  } finally {
    isLoading.value = false
  }
}

function onChecked(workflowId: string) {
  if (workflowIds.value.includes(workflowId)) {
    workflowIds.value = workflowIds.value.filter((id) => id !== workflowId)
  } else {
    workflowIds.value.push(workflowId)
  }
}

function onClear() {
  deleteId.value = undefined
}
</script>
<template>
  <div class="grid grid-cols-2 gap-4">
    <Card>
      <CardHeader>
        <CardTitle>Edit Trait</CardTitle>
      </CardHeader>
      <CardContent>
        <div>
          <Label for="name">Name</Label>
          <Input id="name" type="text" placeholder="Name" v-model="name" />
        </div>
        <div class="mt-4">
          <Label for="description">Description</Label>
          <Input
            id="description"
            type="text"
            placeholder="Description"
            v-model="description"
          />
        </div>
        <div class="mt-4">
          <Label>Content Types</Label>
          <TagsInput v-model="contentTypes">
            <TagsInputItem
              v-for="item in contentTypes"
              :key="item"
              :value="item"
            >
              <TagsInputItemText />
              <TagsInputItemDelete />
            </TagsInputItem>
            <TagsInputInput placeholder="Content Types" />
          </TagsInput>
        </div>
        <div class="mt-4">
          <Label for="execute-on-delete">Execute on Delete</Label>
          <div class="flex items-center gap-2">
            <Select id="execute-on-delete" v-model="deleteId">
              <SelectTrigger class="w-full">
                <SelectValue
                  placeholder="Select a workflow to execute when a collection or metadata is deleted"
                />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectLabel>Workflows</SelectLabel>
                  <SelectItem
                    v-for="workflow in workflows"
                    :value="workflow.id"
                    :key="workflow.id"
                  >
                    {{ workflow.name }}
                  </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
            <Button
              variant="outline"
              size="sm"
              class="bg-transparent"
              @click="onClear"
              v-if="deleteId"
            >
              <Icon name="i-lucide-x" />
            </Button>
          </div>
        </div>
        <div class="mt-8">
          <Button @click="onSubmit" :disabled="isLoading">
            <Icon name="i-lucide-save" class="mr-2" />
            Save
          </Button>
        </div>
      </CardContent>
    </Card>
    <Card>
      <CardHeader>
        <CardTitle>Workflows</CardTitle>
        <CardDescription>Select the workflows associated with this trait.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <div
          v-for="workflow in workflows"
          :key="workflow.id"
          class="flex items-center gap-2 py-2"
        >
          <Checkbox
            :id="'workflow-' + workflow.id"
            :checked="workflowIds.includes(workflow.id)"
            @update:checked="onChecked(workflow.id)"
          />
          <label
            :for="'workflow-' + workflow.id"
            class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
          >
            {{ workflow.name }}
          </label>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
