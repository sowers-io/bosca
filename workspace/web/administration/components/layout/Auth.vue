<script setup lang="ts">
defineProps<{
  reverse?: boolean
}>()
</script>

<template>
  <div
    class="relative flex items-center justify-center px-4 h-dvh lg:max-w-none lg:px-0"
    :class="{ 'flex-row-reverse': reverse }"
  >
    <div
      class="relative hidden h-full flex-1 flex-col bg-muted p-10 text-white lg:flex dark:border-r"
    >
      <div class="absolute inset-0 bg-zinc-800" />
      <div class="relative z-20 flex items-center text-lg font-medium">
        <img src="/logo.svg" alt="Acme Inc" class="h-12 w-12 mr-4" />
        Bosca
      </div>
      <div class="relative z-20 mt-auto">
        <blockquote class="space-y-2">
          <p class="text-sm">
            Bosca is an AI-powered Content Management, Analytics, and
            Personalization platform built to help organizations unlock the full
            potential of their content strategies. With cutting-edge AI/ML
            workflows, Bosca simplifies content management while delivering
            actionable insights and personalized user experiences that drive
            meaningful engagement.
          </p>
        </blockquote>
      </div>
    </div>
    <div class="mx-auto flex-1 lg:p-8">
      <slot />
    </div>
  </div>
</template>
