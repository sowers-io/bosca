<script setup lang="ts">
import { Button } from '~/components/ui/button'
import { Input } from '~/components/ui/input'
import { Label } from '~/components/ui/label'
import {
  Sheet,
  SheetClose,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from '~/components/ui/sheet'
import { toast } from '~/components/ui/toast'
import { CollectionType } from '~/lib/graphql/graphql'

const props = defineProps<{
  parent: string | undefined
}>()

const client = useBoscaClient()
const name = ref('')
const loading = ref(false)

async function onSubmit(e: Event) {
  e.preventDefault()
  e.stopPropagation()
  try {
    loading.value = true
    if (!name.value || name.value.trim().length === 0) {
      throw new Error('Name is required')
    }
    await client.collections.add({
      parentCollectionId: props.parent,
      name: name.value,
      collectionType: CollectionType.Folder,
    })
    toast({
      title: 'Collection Added',
    })
  } catch (e) {
    toast({
      title: 'Error adding collection',
      description: e.message,
    })
    loading.value = false
  }
}
</script>

<template>
  <Sheet>
    <SheetTrigger as-child>
      <Button variant="outline">
        <Icon name="i-lucide-plus" class="mr-2 size-4" />
        Add Collection
      </Button>
    </SheetTrigger>
    <SheetContent>
      <SheetHeader>
        <SheetTitle>Add Collection</SheetTitle>
        <SheetDescription>
          Add a new Collection to the currently selected Collection.
        </SheetDescription>
      </SheetHeader>
      <div class="grid gap-4 py-4">
        <div class="grid grid-cols-4 items-center gap-4">
          <Label for="name" class="text-right">Name</Label>
          <Input id="name" class="col-span-3" v-model="name" />
        </div>
      </div>
      <SheetFooter>
        <SheetClose as-child>
          <Button @click="onSubmit">
            <Icon name="i-lucide-save" class="mr-2 size-4" />
            Save changes
          </Button>
        </SheetClose>
      </SheetFooter>
    </SheetContent>
  </Sheet>
</template>
