<script setup lang="ts">
const breadcrumbs = useBreadcrumbs()
onMounted(() => {
  breadcrumbs.set([{ title: 'Collections', to: '/collections' }])
})
</script>
<template></template>
