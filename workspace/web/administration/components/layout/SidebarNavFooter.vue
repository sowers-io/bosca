<script setup lang="ts">
import { useSidebar } from '~/components/ui/sidebar'
import type { Profile } from '~/lib/graphql/graphql'

const { isMobile, setOpenMobile } = useSidebar()
const client = useBoscaClient()
const profile = await client.profiles.getCurrentProfile()
const avatarUrl = computed(() => {
  return '#'
})
const fallbackName = computed(() =>
  profile?.name?.split(' ')?.map((n) => n[0])?.join('') || ''
)
</script>

<template>
  <SidebarMenu>
    <SidebarMenuItem>
      <DropdownMenu>
        <DropdownMenuTrigger as-child>
          <SidebarMenuButton
            size="lg"
            class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
          >
            <Avatar class="h-8 w-8 rounded-lg">
              <AvatarImage :src="avatarUrl" :alt="profile.name" />
              <AvatarFallback class="rounded-lg">{{
                fallbackName
              }}</AvatarFallback>
            </Avatar>
            <div class="grid flex-1 text-left text-sm leading-tight">
              <span class="truncate font-semibold">{{ profile.name }}</span>
            </div>
            <Icon
              name="i-lucide-chevrons-up-down"
              class="ml-auto size-4"
            />
          </SidebarMenuButton>
        </DropdownMenuTrigger>
        <DropdownMenuContent
          class="min-w-56 w-[--radix-dropdown-menu-trigger-width] rounded-lg"
          :side="isMobile ? 'bottom' : 'right'"
          align="end"
        >
          <DropdownMenuLabel class="p-0 font-normal">
            <div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
              <Avatar class="h-8 w-8 rounded-lg">
                <AvatarImage :src="avatarUrl" :alt="profile.name" />
                <AvatarFallback class="rounded-lg">{{
                  fallbackName
                }}</AvatarFallback>
              </Avatar>
              <div class="grid flex-1 text-left text-sm leading-tight">
                <span class="truncate font-semibold">{{ profile.name }}</span>
              </div>
            </div>
          </DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuGroup>
            <DropdownMenuItem as-child>
              <NuxtLink
                to="/settings"
                @click="setOpenMobile(false)"
              >
                <Icon name="i-lucide-settings" />
                Settings
              </NuxtLink>
            </DropdownMenuItem>
          </DropdownMenuGroup>
          <DropdownMenuSeparator />
          <DropdownMenuItem>
            <a href="/logout" @click="setOpenMobile(false)">
              <Icon name="i-lucide-log-out" />
              Log out
            </a>
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </SidebarMenuItem>
  </SidebarMenu>
</template>
