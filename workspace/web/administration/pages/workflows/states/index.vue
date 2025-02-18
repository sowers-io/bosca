<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import { GetStatesDocument } from '~/lib/graphql/graphql'

const client = useBoscaClient()
const { data: states } = client.workflows.getStatesAsyncData()

const breadcrumbs = useBreadcrumbs()

onMounted(() => {
  breadcrumbs.set([
    { title: 'Workflows' },
    { title: 'States' },
  ])
})
</script>

<template>
  <Table>
    <TableHeader>
      <TableRow>
        <TableHead>Name</TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      <TableRow v-for="state in states" :key="state.id" class="cursor-pointer">
        <TableCell class="font-medium flex content-center">
          {{ state.name }}
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
