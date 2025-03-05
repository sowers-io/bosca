<script lang="ts" setup>
const breadcrumbs = useBreadcrumbs()
const route = useRoute()

onMounted(() => {
  const links: BreadcrumbLink[] = []
  if (route.query.media) {
    links.push({ title: 'Media', to: '/media' })
  } else if (route.query.document) {
    links.push({ title: 'Document', to: '/content/' + route.params.id })
  } else {
    links.push({
      title: 'Collections',
      to: '/collections' +
        (route.query.parent ? '?id=' + route.query.parent : ''),
    })
  }
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
