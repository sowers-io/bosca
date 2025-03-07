<script lang="ts" setup>
const client = useBoscaClient()

const offset = ref(0)
const limit = ref(25)

const { data: groups } = client.security.getGroups(offset, limit)

const breadcrumbs = useBreadcrumbs()
onMounted(() => {
  breadcrumbs.set([
    { title: 'Principals & Profiles' },
    { title: 'Groups' },
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
      <TableRow v-for="group in groups" :key="group.id">
        <TableCell>{{ group.name }}</TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
