<script setup lang="ts">
import { cn } from '@/lib/utils'
import { toTypedSchema } from '@vee-validate/zod'
import { FieldArray, useForm } from 'vee-validate'
import { h, ref } from 'vue'
import * as z from 'zod'
import { toast } from '~/components/ui/toast'

const verifiedEmails = ref(['m@example.com', 'm@google.com', 'm@support.com'])

const profileFormSchema = toTypedSchema(z.object({
  email: z
    .string({
      required_error: 'Please select an email to display.',
    })
    .email(),
  bio: z.string().max(160, {
    message: 'Bio must not be longer than 160 characters.',
  }).min(4, { message: 'Bio must be at least 2 characters.' }),
}))

const { handleSubmit, resetForm } = useForm({
  validationSchema: profileFormSchema,
  initialValues: {
    bio: '',
  },
})

const onSubmit = handleSubmit((values) => {
  toast({
    title: 'You submitted the following values:',
    description: h('pre', {
      class: 'mt-2 w-[340px] rounded-md bg-slate-950 p-4',
    }, h('code', { class: 'text-white' }, JSON.stringify(values, null, 2))),
  })
})
</script>

<template>
  <div>
    <h3 class="text-lg font-medium">
      Profile
    </h3>
  </div>
  <Separator />
  <form class="space-y-8" @submit="onSubmit">
    <FormField v-slot="{ componentField }" name="username">
      <FormItem>
        <FormLabel>Username</FormLabel>
        <FormControl>
          <Input
            type="text"
            :disabled="true"
            placeholder="shadcn"
            v-bind="componentField"
          />
        </FormControl>
        <FormDescription>
        </FormDescription>
        <FormMessage />
      </FormItem>
    </FormField>

    <FormField v-slot="{ componentField }" name="bio">
      <FormItem>
        <FormLabel>Bio</FormLabel>
        <FormControl>
          <Textarea
            placeholder="Tell us a little bit about yourself"
            v-bind="componentField"
          />
        </FormControl>
        <FormDescription>
        </FormDescription>
        <FormMessage />
      </FormItem>
    </FormField>

    <div class="flex justify-start gap-2">
      <Button type="submit">
        Update profile
      </Button>

      <Button
        type="button"
        variant="outline"
        @click="resetForm"
      >
        Reset form
      </Button>
    </div>
  </form>
</template>
