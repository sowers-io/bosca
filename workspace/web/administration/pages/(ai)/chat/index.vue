<script setup lang="ts">
import { Badge } from '~/components/ui/badge'

import { Button } from '~/components/ui/button'
import { Input } from '~/components/ui/input'
import { Label } from '~/components/ui/label'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '~/components/ui/select'
import { Textarea } from '~/components/ui/textarea'
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '~/components/ui/tooltip'
import {
  Bird,
  Book,
  Bot,
  Code2,
  CornerDownLeft,
  LifeBuoy,
  Mic,
  Paperclip,
  Rabbit,
  Settings,
  Settings2,
  Share,
  SquareTerminal,
  SquareUser,
  Triangle,
  Turtle,
} from 'lucide-vue-next'

const breadcrumbs = useBreadcrumbs()

onMounted(() => {
  breadcrumbs.set([
    { title: 'AI Chat' },
  ])
})
</script>

<template>
  <div class="grid w-full overflow-hidden h-[calc(100dvh-105px)]">
    <div class="flex flex-col">
      <main class="grid flex-1 gap-4 p-4 md:grid-cols-2 lg:grid-cols-3">
        <div
          class="relative hidden h-[calc(100dvh-137px)] overflow-auto flex-col items-start gap-8 md:flex"
        >
          <form class="grid w-full items-start gap-6">
            <fieldset class="grid gap-6 rounded-lg border p-4">
              <legend class="-ml-1 px-1 text-sm font-medium">
                Models
              </legend>
              <div class="grid gap-3">
                <Label for="model">Model</Label>
                <Select>
                  <SelectTrigger
                    id="model"
                    class="items-start [&_[data-description]]:hidden"
                  >
                    <SelectValue placeholder="Select a model" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="genesis">
                      <div class="flex items-start gap-3 text-muted-foreground">
                        <Rabbit class="size-5" />
                        <div class="grid gap-0.5">
                          <p>
                            Neural
                            <span class="font-medium text-foreground">
                              Genesis
                            </span>
                          </p>
                          <p class="text-xs" data-description>
                            Our fastest model for general use cases.
                          </p>
                        </div>
                      </div>
                    </SelectItem>
                    <SelectItem value="explorer">
                      <div class="flex items-start gap-3 text-muted-foreground">
                        <Bird class="size-5" />
                        <div class="grid gap-0.5">
                          <p>
                            Neural
                            <span class="font-medium text-foreground">
                              Explorer
                            </span>
                          </p>
                          <p class="text-xs" data-description>
                            Performance and speed for efficiency.
                          </p>
                        </div>
                      </div>
                    </SelectItem>
                    <SelectItem value="quantum">
                      <div class="flex items-start gap-3 text-muted-foreground">
                        <Turtle class="size-5" />
                        <div class="grid gap-0.5">
                          <p>
                            Neural
                            <span class="font-medium text-foreground">
                              Quantum
                            </span>
                          </p>
                          <p class="text-xs" data-description>
                            The most powerful model for complex computations.
                          </p>
                        </div>
                      </div>
                    </SelectItem>
                  </SelectContent>
                </Select>
              </div>
              <div class="grid gap-3">
                <Label for="temperature">Temperature</Label>
                <Input id="temperature" type="number" placeholder="0.4" />
              </div>
              <div class="grid grid-cols-2 gap-4">
                <div class="grid gap-3">
                  <Label for="top-p">Top P</Label>
                  <Input id="top-p" type="number" placeholder="0.7" />
                </div>
                <div class="grid gap-3">
                  <Label for="top-k">Top K</Label>
                  <Input id="top-k" type="number" placeholder="0.0" />
                </div>
              </div>
            </fieldset>
            <fieldset class="grid gap-6 rounded-lg border p-4">
              <legend class="-ml-1 px-1 text-sm font-medium">
                Storage Systems
              </legend>
              <div class="grid gap-3">
                <Label for="role">System</Label>
                <Select default-value="system">
                  <SelectTrigger>
                    <SelectValue placeholder="Select a role" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="system">
                      System
                    </SelectItem>
                    <SelectItem value="user">
                      User
                    </SelectItem>
                    <SelectItem value="assistant">
                      Assistant
                    </SelectItem>
                  </SelectContent>
                </Select>
              </div>
            </fieldset>
            <fieldset class="grid gap-6 rounded-lg border p-4">
              <legend class="-ml-1 px-1 text-sm font-medium">
                Prompts
              </legend>
              <div class="grid gap-3">
                <Select default-value="system">
                  <SelectTrigger>
                    <SelectValue placeholder="Select a role" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="system">
                      System
                    </SelectItem>
                    <SelectItem value="assistant">
                      Assistant
                    </SelectItem>
                  </SelectContent>
                </Select>
                <Label for="content">Content</Label>
                <Textarea
                  id="content"
                  class="min-h-[9.5rem]"
                />
              </div>
            </fieldset>
          </form>
        </div>
        <div
          class="relative flex h-[calc(100dvh-137px)] flex-col rounded-xl bg-muted/50 p-4 lg:col-span-2"
        >
          <Badge variant="outline" class="absolute right-3 top-3">
            Output
          </Badge>
          <div class="flex-1" />
          <div
            class="relative overflow-hidden rounded-lg border bg-background focus-within:ring-1 focus-within:ring-ring"
          >
            <Label for="message" class="sr-only">
              Message
            </Label>
            <Textarea
              id="message"
              placeholder="Type your message here..."
              class="min-h-12 resize-none border-0 p-3 shadow-none focus-visible:ring-0"
            />
            <div class="flex items-center p-3 pt-0">
              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger as-child>
                    <Button variant="ghost" size="icon">
                      <Paperclip class="size-4" />
                      <span class="sr-only">Attach file</span>
                    </Button>
                  </TooltipTrigger>
                  <TooltipContent side="top">
                    Attach File
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger as-child>
                    <Button variant="ghost" size="icon">
                      <Mic class="size-4" />
                      <span class="sr-only">Use Microphone</span>
                    </Button>
                  </TooltipTrigger>
                  <TooltipContent side="top">
                    Use Microphone
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider>
              <Button type="button" size="sm" class="ml-auto gap-1.5">
                Send Message
                <CornerDownLeft class="size-3.5" />
              </Button>
            </div>
          </div>
        </div>
      </main>
    </div>
  </div>
</template>
