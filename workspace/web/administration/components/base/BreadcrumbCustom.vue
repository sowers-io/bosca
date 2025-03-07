<script setup lang="ts">
import { useBreadcrumbs } from '~/composables/useBreadcrumbs'

const manager = useBreadcrumbs()
const links = manager.links

withDefaults(
  defineProps<{
    separator?: string
  }>(),
  {
    separator: 'i-lucide-chevron-right',
  },
)
</script>

<template>
  <Breadcrumb>
    <BreadcrumbList>
      <template v-for="(link, index) in links" :key="index">
        <BreadcrumbItem>
          <BreadcrumbLink v-if="index !== links.length - 1 && link.to" as-child>
            <NuxtLink :to="link.to">
              {{ link.title }}
            </NuxtLink>
          </BreadcrumbLink>
          <BreadcrumbPage v-else>
            {{ link.title }}
          </BreadcrumbPage>
        </BreadcrumbItem>
        <BreadcrumbSeparator v-if="index < links.length - 1">
          <Icon :name="separator" mode="svg" />
        </BreadcrumbSeparator>
      </template>
    </BreadcrumbList>
  </Breadcrumb>
</template>
