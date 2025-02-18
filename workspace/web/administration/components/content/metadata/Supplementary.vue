<script setup lang="ts">
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '~/components/ui/card'
import type {
  MetadataFragment,
  MetadataSupplementary,
} from '~/lib/graphql/graphql'

const props = defineProps<{
  metadata: MetadataFragment
}>()
const client = useBoscaClient()

async function onDownload(supplementary: MetadataSupplementary) {
  const s = await client.metadata.getSupplementary(
    props.metadata.id,
    supplementary.key,
  )
  document.location = s!.content.urls.download.url
}
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>Supplementary</CardTitle>
      <CardDescription>View the supplementary content for this
        metadata.</CardDescription>
    </CardHeader>
    <CardContent>
      <Table>
        <TableHeader>
          <TableRow>
            <TableCell>Name</TableCell>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow
            v-for="supplementary in metadata.supplementary || []"
            :key="supplementary.key"
            class="cursor-pointer"
            @click="onDownload(supplementary)"
          >
            <TableCell>{{ supplementary.name }}</TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </CardContent>
  </Card>
</template>
