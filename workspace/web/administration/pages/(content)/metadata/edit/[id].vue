<script lang="ts" setup>
const breadcrumbs = useBreadcrumbs()
const route = useRoute()

onMounted(() => {
  const links: BreadcrumbLink[] = [
    route.query.media ? { title: 'Media', to: '/media' } : {
      title: 'Collections',
      to: '/collections/browse' +
        (route.query.parent ? '?id=' + route.query.parent : ''),
    },
  ]
  if (route.query.parent) {
    links.push({ title: '...' })
  }
  links.push({ title: 'Edit' })
  breadcrumbs.set(links)
})
</script>
<template>
  <ContentMetadata :metadata-id="route.params.id.toString()" />
</template>
