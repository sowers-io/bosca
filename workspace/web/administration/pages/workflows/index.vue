<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'

const client = useBoscaClient()
const { data: workflows } = await client.workflows.getAllAsyncData()

const breadcrumbs = useBreadcrumbs()

onMounted(() => {
  breadcrumbs.set([
    { title: 'Workflows' },
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
      <TableRow
        v-for="workflow in workflows"
        :key="workflow.id"
        class="cursor-pointer"
      >
        <TableCell class="font-medium flex content-center">
          {{ workflow.name }}
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
