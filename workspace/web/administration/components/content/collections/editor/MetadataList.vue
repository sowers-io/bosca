<script lang="ts" setup>
import type {
  CollectionFragment,
  MetadataFragment,
} from '~/lib/graphql/graphql.ts'
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

const offset = ref(props.offset)
const count = defineModel('count', { type: Number, default: 0 })

const router = useRouter()
const client = useBoscaClient()
const { data } = await client.collections
  .getCollectionChildMetadataAsyncData(
    props.collection.id,
    offset,
    props.limit,
  )

onUpdated(() => {
  offset.value = props.offset
  count.value = data.value?.count || 0
})

onMounted(() => {
  offset.value = props.offset
  count.value = data.value?.count || 0
})
</script>
<template>
  <div>
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>Name</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow
          v-for="item in data?.metadata || []"
          :key="item.id"
          @click="router.push(`/content/${item.id}`)"
          class="cursor-pointer"
        >
          <TableCell class="font-medium flex content-center">
            <NuxtLink :to="'/collections/' + item.id">{{ item.name }}</NuxtLink>
          </TableCell>
        </TableRow>
      </TableBody>
      <TableFooter>
        <TableRow v-if="(data?.metadata?.length || 0) === 0">
          <TableCell class="font-medium flex content-center">
            No content found.
          </TableCell>
        </TableRow>
      </TableFooter>
    </Table>
  </div>
</template>
