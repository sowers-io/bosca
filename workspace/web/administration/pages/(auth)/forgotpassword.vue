<script setup lang="ts">
definePageMeta({
  layout: 'blank',
})
const client = useBoscaClient()
const adminOverrides = await client.configurations.getConfiguration(
  'admin.overrides',
)
</script>

<template>
  <div
    class="flex flex-col items-center justify-center gap-6 bg-muted p-6 min-h-svh md:p-10"
  >
    <div class="max-w-sm w-full flex flex-col gap-6">
      <NuxtLink
        to="/"
        class="flex items-center self-center gap-2 font-bold"
      >
        <img
          src="/logo.svg"
          alt="logo"
          class="size-6"
          v-if="!adminOverrides?.value?.logo?.slug"
        />
        <img
          :src="'/content/image?slug=' + adminOverrides.value.logo.slug"
          alt="logo"
          class="size-6"
          v-else
        />
        <span class="ml-3 font-bold">{{
          adminOverrides?.value?.title?.replace(' ', '&nbsp;') ||
          'Bosca'
        }}</span>
      </NuxtLink>
      <Card cla>
        <CardHeader class="text-center">
          <CardTitle class="text-xl">
            Forgot Password
          </CardTitle>
          <CardDescription>
            Enter your email below to reset your password
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div class="grid mx-auto max-w-sm gap-6 w-full">
            <AuthForgotPassword />
            <p class="text-center text-sm text-muted-foreground">
              Already have an account?
              <NuxtLink
                to="/login"
                class="underline underline-offset-4 hover:text-primary"
              >Login</NuxtLink>
            </p>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>
