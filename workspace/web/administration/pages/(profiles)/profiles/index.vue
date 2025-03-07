<script lang="ts" setup>
const client = useBoscaClient()

const offset = ref(0)
const limit = ref(25)

const { data: profiles } = client.profiles.getProfiles(offset, limit)

const breadcrumbs = useBreadcrumbs()
onMounted(() => {
  breadcrumbs.set([
    { title: 'Principals & Profiles' },
    { title: 'Profiles' },
  ])
})
</script>
<template>
  <Table>
    <TableHeader>
      <TableRow>
        <TableHead>Slug</TableHead>
        <TableHead>Name</TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      <TableRow
        v-for="(profile, index) in profiles"
        :key="profile.slug || index.toString()"
      >
        <TableCell>{{ profile.slug }}</TableCell>
        <TableCell>{{ profile.name }}</TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
