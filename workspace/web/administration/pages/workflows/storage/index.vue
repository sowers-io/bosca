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
const { data: systems } = client.workflows.getStorageSystemsAsyncData()

const breadcrumbs = useBreadcrumbs()

onMounted(() => {
  breadcrumbs.set([
    { title: 'Workflows' },
    { title: 'Storage Systems' },
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
        v-for="system in systems"
        :key="system.id"
        class="cursor-pointer"
      >
        <TableCell class="font-medium flex content-center">
          {{ system.name }}
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
