<script lang="ts" setup>
import type { CollectionFragment } from '~/lib/graphql/graphql.ts'
import Table from '~/components/ui/table/Table.vue'
import TableHeader from '~/components/ui/table/TableHeader.vue'
import TableHead from '~/components/ui/table/TableHead.vue'
import TableRow from '~/components/ui/table/TableRow.vue'
import TableBody from '~/components/ui/table/TableBody.vue'
import TableFooter from '~/components/ui/table/TableFooter.vue'

const props = defineProps<{
  collection: CollectionFragment
  limit: number
  offset: number
  count: number
}>()

const count = defineModel('count', { type: Number, default: 0 })
const offset = defineModel('offset', { type: Number, default: 0 })

const router = useRouter()
const client = useBoscaClient()
const { collections, count: collectionCount } = await client.collections
  .getCollectionChildCollections(
    props.collection.id,
    offset,
    props.limit,
  )

onUpdated(() => {
  count.value = collectionCount
})
onMounted(() => {
  count.value = collectionCount
})
</script>
<template>
  <div class="w-full">
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>Name</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow
          v-for="item in collections"
          :key="item.id"
          @click="router.push(`/collections/${item.id}`)"
          class="cursor-pointer"
        >
          <TableCell class="font-medium flex content-center">
            <NuxtLink :to="'/collections/' + item.id">{{ item.name }}</NuxtLink>
          </TableCell>
        </TableRow>
      </TableBody>
      <TableFooter>
        <TableRow v-if="collections.length === 0">
          <TableCell class="font-medium flex content-center">
            No collections found.
          </TableCell>
        </TableRow>
      </TableFooter>
    </Table>
  </div>
</template>
