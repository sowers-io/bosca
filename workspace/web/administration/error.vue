<script setup lang="ts">
import type { NuxtError } from '#app'

const { theme, radius } = useCustomize()

useHead({
  bodyAttrs: {
    class: computed(() => `theme-${theme.value}`),
    style: computed(() => `--radius: ${radius.value}rem;`),
  },
})

const props = defineProps({
  error: Object as () => NuxtError,
})
console.error(props.error)

const router = useRouter()
</script>

<template>
  <div class="h-svh">
    <div
      class="m-auto h-full w-full flex flex-col items-center justify-center gap-2"
    >
      <h1 class="text-[7rem] font-bold leading-tight">
        {{ error?.statusCode || 404 }}
      </h1>
      <template v-if="error?.statusCode === 500">
        <span class="font-medium">Oops! Something went wrong!</span>
        <p class="text-center text-muted-foreground">
          There was an error executing your request.
        </p>
        <p
          class="text-center text-muted-foreground overflow-auto my-10 max-w-[500px]"
        >
          {{ error }}
        </p>
      </template>
      <template v-else>
        <span class="font-medium">Oops! Page Not Found!</span>
        <p class="text-center text-muted-foreground">
          It seems like the page you're looking for <br>
          does not exist or might have been removed.
        </p>
      </template>
      <div class="mt-6 flex gap-4">
        <Button variant="outline" @click="router.back()">
          Go Back
        </Button>
        <Button @click="router.push('/')">
          Back to Home
        </Button>
      </div>
    </div>
  </div>
</template>
