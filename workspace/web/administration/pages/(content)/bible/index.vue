<script lang="ts" setup>
import { toast } from '~/components/ui/toast'

const dropZoneRef = ref<HTMLDivElement>()
const client = useBoscaClient()

const rawBiblesCollections = await client.collections.findCollection(
  [ { attributes: [ { key: 'collection', value: 'raw-bibles' } ] } ],
  0,
  1,
)
const rawBiblesCollectionId = rawBiblesCollections[0].id

const biblesCollections = await client.collections.findCollection(
    [ { attributes: [ { key: 'collection', value: 'bibles' } ] } ],
  0,
  1,
)
const biblesCollectionId = biblesCollections[0].id

const { data: rawBibles, refresh: refreshRawBibles } = await client.collections.listAsyncData(rawBiblesCollectionId)
const { data: bibles, refresh } = await client.collections.listAsyncData(biblesCollectionId)

const router = useRouter()

async function onDrop(files: File[] | null) {
  if (!files) return
  toast({
    title: 'Uploading files, please wait...',
  })
  try {
    const traitIds: string[][] = []
    for (const file of files) {
      if (!file.type.endsWith('/zip')) {
        toast({
          title: 'Unsupported file: ' + file.name,
        })
        return
      }
      traitIds.push(['bible.usx'])
    }
    const ids = await client.metadata.addFiles(
      rawBiblesCollectionId,
      files,
      traitIds,
    )
    for (const id of ids) {
      await client.metadata.setReady(id)
    }
    toast({
      title: 'File(s) uploaded',
    })
  } catch (e) {
    console.error('Error uploading file(s)', e)
    toast({
      title: 'Error uploading file(s)',
      description: (e as unknown as any).message,
    })
  }
}

useDropZone(dropZoneRef, {
  onDrop,
  multiple: true,
  preventDefaultForUnhandled: false,
})
client.listeners.onCollectionChanged((id) => {
  if (id === biblesCollectionId) {
    refresh()
  }
  if (id === rawBiblesCollectionId) {
    refreshRawBibles()
  }
})

const breadcrumbs = useBreadcrumbs()

onMounted(() => {
  breadcrumbs.set([
    { title: 'Bible' },
  ])
})
</script>
<template>
  <div ref="dropZoneRef" class="h-full w-full flex flex-wrap md:flex-nowrap gap-4">
    <Card class="w-full min-w-[600px]">
      <CardContent>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Name</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow
              v-for="bible in bibles?.items || []"
              :key="bible.id"
              @click="
                router.push(
                  '/collections/edit/' + bible.id + '?bible=true',
                )
              "
              class="cursor-pointer"
            >
              <TableCell>
                {{ bible.name }}
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </CardContent>
    </Card>
    <Card class="min-w-[300px] w-full md:w-auto">
      <CardContent>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Raw Bibles</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow
              v-for="bible in rawBibles?.items || []"
              :key="bible.id"
              @click="
                router.push(
                  '/metadata/edit/' + bible.id,
                )
              "
              class="cursor-pointer"
            >
              <TableCell>
                {{ bible.name }}
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  </div>
</template>
