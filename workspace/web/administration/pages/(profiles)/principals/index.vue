<script lang="ts" setup>
const client = useBoscaClient()

const offset = ref(0)
const limit = ref(25)

const { data: principals } = client.security.getPrincipals(offset, limit)

const breadcrumbs = useBreadcrumbs()
onMounted(() => {
  breadcrumbs.set([
    { title: 'Principals & Profiles' },
    { title: 'Principals' },
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
      <TableRow v-for="principal in principals" :key="principal.id">
        <TableCell>{{ principal.id }}</TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
