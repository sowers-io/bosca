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
const { data: models } = client.workflows.getModelsAsyncData()

const breadcrumbs = useBreadcrumbs()

onMounted(() => {
  breadcrumbs.set([
    { title: 'Workflows' },
    { title: 'Models' },
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
      <TableRow v-for="model in models" :key="model.id" class="cursor-pointer">
        <TableCell class="font-medium flex content-center">
          {{ model.name }}
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
