<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { GetTraitsDocument } from '~/lib/graphql/graphql'

const client = useBoscaClient()
const router = useRouter()
const { data: traits, refresh } = client.workflows.getTraitsAsyncData()

client.listeners.onTraitChanged(() => {
  refresh()
})

const breadcrumbs = useBreadcrumbs()

onMounted(() => {
  breadcrumbs.set([
    { title: 'Workflows' },
    { title: 'Traits' },
  ])
})
</script>

<template>
  <Table>
    <TableHeader>
      <TableRow>
        <TableHead>Name</TableHead>
        <TableHead>Description</TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      <TableRow
        v-for="trait in traits"
        :key="trait.id"
        class="cursor-pointer"
        @click="router.push(`/workflows/traits/edit/${trait.id}`)"
      >
        <TableCell class="font-medium flex content-center">
          {{ trait.name }}
        </TableCell>
        <TableCell>
          {{ trait.description }}
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
