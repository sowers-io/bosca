<script setup lang="ts">
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '~/components/ui/card'
import type {
  CollectionFragment,
  MetadataFragment,
} from '~/lib/graphql/graphql'

const props = defineProps<{
  content: CollectionFragment | MetadataFragment
}>()
const client = useBoscaClient()

function getPlans() {
  if (props.content.version) {
    return client.workflows.getMetadataWorkflowPlansAsyncData(props.content.id)
  }
  return client.workflows.getCollectionWorkflowPlansAsyncData(props.content.id)
}

const { data: plans, refresh } = getPlans()

if (props.content.version) {
  client.listeners.onMetadataChanged((id) => {
    if (props.content.id === id) {
      refresh()
    }
  })
} else {
  client.listeners.onCollectionChanged((id) => {
    if (props.content.id === id) {
      refresh()
    }
  })
}
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Workflows</CardTitle>
      <CardDescription>View the workflows for this {{
          content.version ? 'Metadata' : 'Collection'
        }}.</CardDescription>
    </CardHeader>
    <CardContent>
      <Table>
        <TableHeader>
          <TableRow>
            <TableCell>Name</TableCell>
            <TableCell>Active</TableCell>
            <TableCell>Failed</TableCell>
            <TableCell>Complete</TableCell>
            <TableCell>Error</TableCell>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow
            v-for="plan in plans || []"
            :key="plan.id.id + '-' + plan.id.queue"
          >
            <TableCell>{{ plan.workflow.name }}</TableCell>
            <TableCell>{{ plan.active.length }}</TableCell>
            <TableCell>{{ plan.failed.length }}</TableCell>
            <TableCell>{{ plan.complete.length }}</TableCell>
            <TableCell>{{ plan.error || '--' }}</TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </CardContent>
  </Card>
</template>
