<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'

const router = useRouter()
const client = useBoscaClient()
const { data: prompts } = client.workflows.getPromptsAsyncData()

const breadcrumbs = useBreadcrumbs()

onMounted(() => {
  breadcrumbs.set([
    { title: 'Workflows' },
    { title: 'Prompts' },
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
        v-for="prompt in prompts"
        :key="prompt.id"
        class="cursor-pointer"
        @click="router.push(`/workflows/prompts/edit/${prompt.id}`)"
      >
        <TableCell class="font-medium flex content-center">
          {{ prompt.name }}
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
