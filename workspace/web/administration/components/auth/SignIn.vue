<script setup lang="ts">
import { Loader2 } from 'lucide-vue-next'
import PasswordInput from '~/components/PasswordInput.vue'
import { LoginDocument } from '~/lib/graphql/graphql'
import { useToast } from '~/components/ui/toast'

const identifier = ref('')
const password = ref('')
const isLoading = ref(false)
const { toast } = useToast()
const client = useBoscaClient()

async function onSubmit(event: Event) {
  event.preventDefault()
  if (!identifier.value || !password.value) {
    return
  }
  isLoading.value = true
  try {
    await client.security.loginWithPassword(identifier.value, password.value)
    navigateTo('/')
  } catch (e: any) {
    toast({
      title: 'Login Failed',
      description: e.message,
    })
  } finally {
    isLoading.value = false
  }
}

function onContinueWithGoogle() {
  toast({
    title: 'Not Implemented',
  })
}
</script>

<template>
  <form class="grid gap-6" @submit="onSubmit">
    <div class="flex flex-col gap-4">
      <Button
        variant="outline"
        class="w-full gap-2"
        @click="onContinueWithGoogle"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          class="size-4"
        >
          <path
            d="M12.48 10.92v3.28h7.84c-.24 1.84-.853 3.187-1.787 4.133-1.147 1.147-2.933 2.4-6.053 2.4-4.827 0-8.6-3.893-8.6-8.72s3.773-8.72 8.6-8.72c2.6 0 4.507 1.027 5.907 2.347l2.307-2.307C18.747 1.44 16.133 0 12.48 0 5.867 0 .307 5.387.307 12s5.56 12 12.173 12c3.573 0 6.267-1.173 8.373-3.36 2.16-2.16 2.84-5.213 2.84-7.667 0-.76-.053-1.467-.173-2.053H12.48z"
            fill="currentColor"
          />
        </svg>
        Continue with Google
      </Button>
    </div>
    <Separator>Or continue with</Separator>
    <div class="grid gap-2">
      <Label for="email">
        Email
      </Label>
      <Input
        id="email"
        v-model="identifier"
        placeholder="name@example.com"
        :disabled="isLoading"
        auto-capitalize="none"
        auto-complete="email"
        auto-correct="off"
      />
    </div>
    <div class="grid gap-2">
      <div class="flex items-center">
        <Label for="password">
          Password
        </Label>
      </div>
      <PasswordInput id="password" v-model="password" />
    </div>
    <Button type="submit" class="w-full" :disabled="isLoading">
      <Loader2 v-if="isLoading" class="mr-2 h-4 w-4 animate-spin" />
      Continue
    </Button>
    <NuxtLink
      to="/forgotpassword"
      class="ml-auto inline-block text-sm underline text-muted-foreground hover:text-primary"
    >
      Forgot your password?
    </NuxtLink>
  </form>
  <div class="mt-8 text-center text-sm text-muted-foreground">
    Don't have an account?
    <NuxtLink to="/signup" class="underline hover:text-primary">
      Create an Account
    </NuxtLink>
  </div>
</template>
