<script setup lang="ts">
import type { NuxtError } from '#app'

const props = defineProps<{
  error: NuxtError
}>()

useHead({
  htmlAttrs: {
    lang: 'en'
  }
})

useSeoMeta({
  title: 'Page not found',
  description: 'We are sorry but this page could not be found.'
})

console.log(props.error)
</script>

<template>
  <UApp>
    <AppHeader />

    <UError :error="error" />

    <AppFooter />
  </UApp>
</template>
