<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table'
import {
  GetStatesDocument,
  GetTransitionsDocument,
  type TraitFragment,
  type WorkflowState,
} from '~/lib/graphql/graphql'

const client = useBoscaClient()
const { data: transitions } = client.workflows.getTransitionsAsyncData()
const { data: states } = client.workflows.getStatesAsyncData()

const statesById = computed(() => {
  const statesById: { [id: string]: WorkflowState } = {}
  for (const state of states.value || []) {
    statesById[state.id] = state
  }
  return statesById
})

const breadcrumbs = useBreadcrumbs()

onMounted(() => {
  breadcrumbs.set([
    { title: 'Workflows' },
    { title: 'Transitions' },
  ])
})
</script>

<template>
  <Table>
    <TableHeader>
      <TableRow>
        <TableHead>Transition</TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      <TableRow
        v-for="transition in transitions"
        :key="transition.fromStateId + '-' + transition.toStateId"
        class="cursor-pointer"
      >
        <TableCell class="font-medium flex content-center">
          {{
            statesById[transition.fromStateId]?.name ||
            transition.fromStateId
          }} ->
          {{
            statesById[transition.toStateId]?.name ||
            transition.toStateId
          }}
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
