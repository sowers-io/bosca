<script setup lang="ts">
definePageMeta({
  layout: 'blank',
})
const cookie = useCookie('_bat')
const profile = useCookie('_profile')

onMounted(async () => {
  cookie.value = null
  profile.value = null
  setTimeout(() => {
    navigateTo('/login')
  }, 10)
})
</script>

<template>
  <div></div>
</template>
