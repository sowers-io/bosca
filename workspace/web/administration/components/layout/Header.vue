<script setup lang="ts">
const breadcrumbs = useBreadcrumbs()
const links = breadcrumbs.links
</script>

<template>
  <header
    class="sticky top-0 z-10 h-[53px] flex items-center gap-4 border-b bg-background px-2 md:px-4"
  >
    <div class="w-full flex items-center gap-4">
      <SidebarTrigger />
      <Separator orientation="vertical" class="h-4" />
      <Breadcrumb>
        <BreadcrumbList>
          <template v-for="(item, index) in links" :key="index.toString()">
            <BreadcrumbItem>
              <NuxtLink
                :to="item.to"
                v-if="item.to"
                class="transition-colors hover:text-foreground text-muted-foreground"
              >
                {{ item.title }}
              </NuxtLink>
              <span
                v-else-if="index !== links.length - 1"
                class="text-muted-foreground"
              >{{ item.title }}</span>
              <span v-else class="text-foreground">{{ item.title }}</span>
            </BreadcrumbItem>
            <BreadcrumbSeparator v-if="index !== links.length - 1" />
          </template>
        </BreadcrumbList>
      </Breadcrumb>
    </div>
    <div class="ml-auto">
      <slot />
    </div>
  </header>
</template>
