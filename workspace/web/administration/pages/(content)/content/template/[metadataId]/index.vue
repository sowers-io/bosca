<script setup lang="ts">
const breadcrumbs = useBreadcrumbs()

onMounted(() => {
  breadcrumbs.set([
    { title: 'Content', to: '/content' },
    { title: 'Edit Template' },
  ])
})
</script>

<template>
  TODO: Edit Template
</template>
