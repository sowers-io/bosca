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
  CollectionSupplementaryFragment,
} from '~/lib/graphql/graphql'

const props = defineProps<{
  collection: CollectionFragment
}>()
const client = useBoscaClient()

async function onDownload(supplementary: CollectionSupplementaryFragment) {
  const s = await client.collections.getSupplementary(
    props.collection.id,
    supplementary.key,
  )
  document.location = s!.content.urls.download.url
}
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Supplementary</CardTitle>
      <CardDescription>View the supplementary content for this collection.</CardDescription>
    </CardHeader>
    <CardContent>
      <Table>
        <TableHeader>
          <TableRow>
            <TableCell>Key</TableCell>
            <TableCell>Name</TableCell>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow
            v-for="supplementary in collection.supplementary || []"
            :key="supplementary.key"
            class="cursor-pointer"
            @click="onDownload(supplementary)"
          >
            <TableCell>{{ supplementary.key }}</TableCell>
            <TableCell>{{ supplementary.name }}</TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </CardContent>
  </Card>
</template>
