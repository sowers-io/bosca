<script setup lang="ts">
import VerifySignUp from '~/components/auth/VerifySignUp.vue'

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
      <div class="flex flex-col gap-6">
        <Card>
          <CardHeader class="text-center">
            <CardTitle class="text-xl">
              Verify Your Account
            </CardTitle>
            <CardDescription>
              Enter your verification code below.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <VerifySignUp />
            <p class="text-center text-sm text-muted-foreground mt-6">
              Already have an account?
              <NuxtLink
                to="/login"
                class="underline underline-offset-4 hover:text-primary"
              >Login</NuxtLink>
            </p>
          </CardContent>
        </Card>
        <div
          class="text-center text-balance text-xs text-muted-foreground [&_a]:underline [&_a]:underline-offset-4"
        >
          By clicking continue, you agree to our <a
            href="/terms"
            class="hover:text-primary"
          >Terms of Service</a>
          and <a href="/privacy" class="hover:text-primary">Privacy Policy</a>.
        </div>
      </div>
    </div>
  </div>
</template>
