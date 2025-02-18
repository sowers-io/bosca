<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '~/components/ui/table'
import type { CollectionItem } from '~/lib/bosca/contentcollection'
import { toast } from '~/components/ui/toast'
import type { CollectionFragment } from '~/lib/graphql/graphql'

const props = defineProps<{
  collection: CollectionFragment
  items: CollectionItem[]
}>()

const client = useBoscaClient()
const router = useRouter()

async function setReady(item: CollectionItem) {
  try {
    await client.metadata.setReady(item.id)
    toast({
      title: 'Metadata marked as ready',
    })
  } catch (e) {
    console.error('Error setting item ready', e)
    toast({
      title: 'Error setting item ready',
      description: (e as unknown as any).message,
    })
  }
}

async function onDelete(item: CollectionItem) {
  try {
    switch (item.__typename) {
      case 'Collection':
        await client.collections.delete(item.id)
        break
      case 'Metadata':
        await client.metadata.delete(item.id)
        break
    }
  } catch (e) {
    console.error('Error Deleting Item', e)
    toast({
      title: 'Error Deleting Item',
      description: (e as unknown as any).message,
    })
  }
}

function onEdit(item: CollectionItem) {
  switch (item.__typename) {
    case 'Collection':
      router.push(
        '/collections/edit/' + item.id + '?parent=' +
          props.collection.id,
      )
      break
    case 'Metadata':
      router.push(
        '/metadata/edit/' + item.id + '?parent=' +
          props.collection.id,
      )
      break
  }
}

function onClick(item: CollectionItem) {
  switch (item.__typename) {
    case 'Collection':
      router.push(
        '/collections?id=' + item.id + '&parent=' + props.collection.id,
      )
      break
    case 'Metadata':
      onEdit(item)
      break
  }
}
</script>

<template>
  <Table>
    <TableHeader>
      <TableRow>
        <TableHead>Name</TableHead>
        <TableHead class="text-center w-48">Actions</TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      <TableRow
        v-for="item in items"
        :key="item.id"
        class="cursor-pointer"
      >
        <TableCell
          class="flex font-medium gap-2"
          @click="onClick(item)"
        >
          <Icon
            name="i-lucide-folder"
            class="size-4 mr-2"
            v-if="item.__typename === 'Collection'"
          />
          <Icon
            name="i-lucide-file"
            class="size-4 mr-2"
            v-if="item.__typename === 'Metadata'"
          />
          {{ item.name }}
        </TableCell>
        <TableCell class="text-center">
          <Button
            variant="ghost"
            size="sm"
            @click="setReady(item)"
            v-if="item.__typename === 'Metadata' && !item.ready"
          >
            <Icon name="i-lucide-check" class="size-4" />
          </Button>
          <Button variant="ghost" size="sm" @click="onEdit(item)">
            <Icon name="i-lucide-pencil" class="size-4" />
          </Button>
          <Button variant="ghost" size="sm" @click="onDelete(item)">
            <Icon name="i-lucide-trash" class="size-4" />
          </Button>
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
