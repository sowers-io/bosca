<script setup lang="ts">
import { Loader2 } from 'lucide-vue-next'
import { toast } from '~/components/ui/toast'

const route = useRoute()
const client = useBoscaClient()
const isLoading = ref(false)
const token = ref(route.query.token as string | undefined)

async function onSubmit(event: Event) {
  event.preventDefault()
  event.stopImmediatePropagation()
  isLoading.value = true
  try {
    await client.security.verify(token.value || '')
    toast({
      title: 'Verify succeeded, please login.',
    })
    navigateTo('/login')
  } catch (e: any) {
    toast({
      title: 'Verification Failed',
      description: e.message,
    })
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <form @submit="onSubmit">
    <div class="grid gap-4">
      <div class="grid gap-2">
        <Label for="email">
          Verification Code
        </Label>
        <Input
          id="code"
          placeholder="Verification Code"
          :disabled="isLoading"
          v-model="token"
        />
      </div>
      <Button :disabled="isLoading">
        <Loader2 v-if="isLoading" class="mr-2 h-4 w-4 animate-spin" />
        Verify
      </Button>
    </div>
  </form>
</template>
