<script lang="ts" setup>
const breadcrumbs = useBreadcrumbs()

onMounted(() => {
  breadcrumbs.set([
    { title: 'Media' },
  ])
})
</script>
<template>
  <ContentMedia
    :filter="
      reactive({
        jpg: true,
        png: true,
        webp: true,
        mp4: true,
        mp3: true,
        webm: true,
        youtube: true,
      })
    "
  />
</template>
