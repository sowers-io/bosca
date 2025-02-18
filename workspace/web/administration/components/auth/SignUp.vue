<script setup lang="ts">
import { cn } from '@/lib/utils'
import { Loader2 } from 'lucide-vue-next'
import PasswordInput from '~/components/PasswordInput.vue'
import { ProfileVisibility } from '~/lib/graphql/graphql'
import { toast } from '~/components/ui/toast'

const router = useRouter()
const isLoading = ref(false)

const client = useBoscaClient()
const name = ref('')
const email = ref('')
const password = ref('')
const confirmPassword = ref('')

async function onSubmit(event: Event) {
  event.preventDefault()
  event.stopImmediatePropagation()
  isLoading.value = true
  try {
    if (name.value.trim().length === 0) {
      throw new Error('Name is required')
    }
    if (email.value.trim().length === 0) {
      throw new Error('Email is required')
    }
    if (email.value.indexOf('@') == -1) {
      throw new Error('Email is invalid')
    }
    if (password.value !== confirmPassword.value) {
      throw new Error('Passwords do not match')
    }
    await client.security.signUpWithPassword(
      {
        name: name.value.trim(),
        visibility: ProfileVisibility.User,
        attributes: [{
          priority: 1,
          source: 'signup',
          typeId: 'bosca.profiles.name',
          visibility: ProfileVisibility.User,
          confidence: 100,
          attributes: {
            name: name.value.trim(),
          },
        }, {
          priority: 1,
          source: 'signup',
          typeId: 'bosca.profiles.email',
          visibility: ProfileVisibility.User,
          confidence: 100,
          attributes: {
            email: email.value.trim().toLocaleLowerCase(),
          },
        }],
      },
      email.value.trim().toLocaleLowerCase(),
      password.value,
    )
    await router.replace('/signup/verify')
  } catch (e) {
    toast({
      title: 'Create Account Failed',
      description: (e as unknown as any).message,
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
  <div :class="cn('grid gap-6', $attrs.class ?? '')">
    <form @submit="onSubmit">
      <div class="grid gap-4">
        <div class="grid gap-2">
          <Label for="name">
            Name
          </Label>
          <Input
            id="name"
            placeholder="Enter your name"
            type="text"
            auto-capitalize="none"
            auto-complete="name"
            auto-correct="off"
            v-model="name"
            :disabled="isLoading"
          />
        </div>
        <div class="grid gap-2">
          <Label for="email">
            Email
          </Label>
          <Input
            id="email"
            placeholder="name@example.com"
            type="email"
            auto-capitalize="none"
            auto-complete="email"
            auto-correct="off"
            v-model="email"
            :disabled="isLoading"
          />
        </div>
        <div class="grid gap-2">
          <Label for="password">
            Password
          </Label>
          <PasswordInput id="password" v-model="password" />
        </div>
        <div class="grid gap-2">
          <Label for="confirm-password">
            Confirm Password
          </Label>
          <PasswordInput id="confirm-password" v-model="confirmPassword" />
        </div>
        <Button :disabled="isLoading">
          <Loader2
            v-if="isLoading"
            class="mr-2 h-4 w-4 animate-spin"
          />
          Continue
        </Button>
      </div>
    </form>
    <Separator label="Or continue with" />
    <div class="flex flex-col gap-4">
      <Button
        variant="outline"
        class="w-full gap-2"
        @click="onContinueWithGoogle"
        :disabled="isLoading"
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
  </div>
</template>
