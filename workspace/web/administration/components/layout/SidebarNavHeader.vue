<script setup lang="ts">
const client = useBoscaClient()
const adminOverrides = await client.configurations.getConfiguration('admin.overrides')
</script>

<template>
  <SidebarMenu>
    <SidebarMenuItem>
      <div class="flex ms-1 my-2">
        <img
          src="/logo.svg"
          alt="logo"
          class="w-6 h-6"
          v-if="!adminOverrides?.value?.logo?.slug"
        />
        <img
          :src="'/content/image?slug=' + adminOverrides.value.logo.slug"
          alt="logo"
          class="w-6 h-6"
          v-else
        />
        <span class="ml-3 font-bold">{{ adminOverrides?.value?.title || 'Bosca' }}</span>
      </div>
    </SidebarMenuItem>
  </SidebarMenu>
</template>
