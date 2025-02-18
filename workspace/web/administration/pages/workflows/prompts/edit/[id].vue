<script setup lang="ts">
import { Button } from '~/components/ui/button'
import { Label } from '~/components/ui/label'
import { Separator } from '~/components/ui/separator'
import { Textarea } from '~/components/ui/textarea'
import MaxLengthSelector from '~/components/prompts/MaxLengthSelector.vue'

import ModelSelector from '~/components/prompts/ModelSelector.vue'
import TemperatureSelector from '~/components/prompts/TemperatureSelector.vue'
import TopPSelector from '~/components/prompts/TopPSelector.vue'
import { toast } from '~/components/ui/toast'

const route = useRoute()
const client = useBoscaClient()
const isLoading = ref(false)
const { data: prompt } = route.params.id === 'new'
  ? { data: ref({}) }
  : client.workflows.getPromptAsyncData(route.params.id.toString())

const name = ref(prompt.value?.name)
const description = ref(prompt.value?.description)
const inputType = ref(prompt.value?.inputType)
const outputType = ref(prompt.value?.outputType)
const systemPrompt = ref(prompt.value?.systemPrompt)
const userPrompt = ref(prompt.value?.userPrompt)

function onChanged() {
  const p = prompt.value
  if (!p) return
  name.value = p.name
  description.value = p.description
  inputType.value = p.inputType
  outputType.value = p.outputType
  systemPrompt.value = p.systemPrompt
  userPrompt.value = p.userPrompt
}

watch(prompt, () => {
  onChanged()
})

async function onSave() {
  isLoading.value = true
  try {
    const p = {
      name: name.value,
      description: description.value,
      systemPrompt: systemPrompt.value,
      userPrompt: userPrompt.value,
      inputType: inputType.value,
      outputType: outputType.value,
    }

    if (prompt.value?.id) {
      await client.workflows.editPrompt(prompt.value!.id, p)
    } else {
      const id = await client.workflows.addPrompt(p)
      navigateTo(`/workflows/prompts/edit/${id}`)
    }

    toast({
      title: 'Prompt saved',
    })
    navigateTo('/workflows/prompts')
  } catch (e) {
    console.error(e)
    toast({
      title: 'Failed to save prompt',
      description: (e as unknown as any).message,
    })
  } finally {
    isLoading.value = false
  }
}

async function onDelete() {
  isLoading.value = true
  try {
    await client.workflows.deletePrompt(prompt.value!.id)
    toast({
      title: 'Prompt deleted',
    })
    navigateTo('/workflows/prompts')
  } catch (e) {
    console.error(e)
    toast({
      title: 'Failed to delete prompt',
      description: (e as unknown as any).message,
    })
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div class="hidden h-full flex-col md:flex">
    <div
      class="container flex flex-col items-start justify-between space-y-2 py-4 sm:flex-row sm:items-center sm:space-y-0 md:h-16"
    >
      <div class="container py-6">
        <Label for="name">Name</Label>
        <Input id="name" v-model="name" placeholder="Prompt Name" />
      </div>
      <div class="ml-auto flex w-full space-x-2 sm:justify-end">
        <Button variant="ghost" @click="onSave" :disabled="isLoading">
          <Icon name="i-lucide-save" class="size-4" />
        </Button>
        <Button variant="ghost" @click="onDelete" :disabled="isLoading">
          <Icon name="i-lucide-trash" class="size-4" />
        </Button>
      </div>
    </div>
    <Separator class="mt-4" />
    <div class="container py-2">
      <Label for="description">Description</Label>
      <Input id="description" v-model="description" placeholder="Description" />
    </div>
    <div class="container py-2">
      <Label for="description">Input Type</Label>
      <Input id="description" v-model="inputType" placeholder="Input Type" />
    </div>
    <div class="container py-2">
      <Label for="description">Output Type</Label>
      <Input id="description" v-model="outputType" placeholder="Output Type" />
    </div>
    <Separator class="mt-4" />
    <div class="container h-full py-6">
      <div
        class="grid h-full items-stretch gap-6 md:grid-cols-[minmax(0,1fr)_200px]"
      >
        <div class="hidden flex-col space-y-4 sm:flex md:order-2">
          <ModelSelector />
          <TemperatureSelector :default-value="[0.56]" />
          <MaxLengthSelector :default-value="[256]" />
          <TopPSelector :default-value="[0.9]" />
        </div>
        <div class="md:order-1">
          <div class="flex flex-col space-y-4">
            <div class="grid h-full gap-6 lg:grid-cols-2">
              <div class="flex flex-col space-y-4">
                <div class="flex flex-1 flex-col space-y-2">
                  <Label for="system-prompt">System Prompt</Label>
                  <Textarea
                    id="system-prompt"
                    placeholder="Enter the System Prompt"
                    class="flex-1 lg:min-h-[280px]"
                    v-model="systemPrompt"
                  />
                </div>
                <div class="flex flex-col space-y-2">
                  <Label for="user-prompt">User Prompt</Label>
                  <Textarea
                    id="user-prompt"
                    class="flex-1 lg:min-h-[280px]"
                    placeholder="Enter the User Prompt"
                    v-model="userPrompt"
                  />
                </div>
                <div class="grid justify-items-end space-x-2">
                  <Button>
                    <Icon name="i-lucide-send" class="size-4" />
                  </Button>
                </div>
              </div>
              <div
                class="mt-[21px] min-h-[400px] rounded-md border bg-muted lg:min-h-[700px]"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
