<script setup lang="ts">
</script>

<template>
  <div class="w-full flex flex-col gap-4">
    <div class="flex flex-wrap items-center justify-between gap-2">
      <h2 class="text-2xl font-bold tracking-tight">
        Dashboard
      </h2>
      <div class="flex items-center space-x-2">
        <BaseDateRangePicker />
      </div>
    </div>
    <main class="flex flex-1 flex-col gap-4 md:gap-8"></main>
  </div>
</template>
