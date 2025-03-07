<template>
  <div
    class="flex border bg-background gap-6 rounded-md p-2 drop-shadow-xl ms-4"
  >
    <div v-if="items.length" @mouseenter="selection = -1" class="w-full">
      <button
        v-for="(item, index) in items as CommandItem[]"
        :key="index.toString() + '-' + item.label"
        :class="
          {
            'bg-accent text-foreground': index === selection,
            'italic font-bold': item.name && editor.isActive(item.name),
            'inline-flex py-2.5 px-2 items-center justify-start rounded-md text-sm font-medium transition-colors hover:bg-zinc-200 hover:text-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 w-full':
              true,
          }
        "
        @click="onSelect(index)"
      >
        <Icon :name="item.icon" class="h-4 w-4 mr-2" />
        {{ item.label }}
        <div class="grow"></div>
        <Icon
          name="i-lucide-check"
          class="h-4 w-4"
          v-if="item.name && editor.isActive(item.name)"
        />
      </button>
    </div>
    <div class="item" v-else>
      No result
    </div>
  </div>
</template>

<script lang="ts">
import type { CommandItem } from '~/lib/editor/commanditems'

export default {
  props: {
    items: {
      type: Array,
      required: true,
    },

    editor: {
      type: Object,
      required: true,
    },

    command: {
      type: Function,
      required: true,
    },
  },

  data() {
    return {
      selection: 0,
    }
  },

  watch: {
    items() {
      this.selection = 0
    },
  },

  methods: {
    onKeyDown({ event }: { event: KeyboardEvent }) {
      switch (event.key) {
        case 'ArrowUp':
          this.upHandler()
          return true
        case 'ArrowRight':
          this.upHandler()
          return true
        case 'ArrowLeft':
          this.downHandler()
          return true
        case 'ArrowDown':
          this.downHandler()
          return true
        case 'Enter':
          this.enterHandler()
          return true
      }
      return false
    },

    upHandler() {
      this.selection = ((this.selection + this.items.length) - 1) %
        this.items.length
    },

    downHandler() {
      this.selection = (this.selection + 1) % this.items.length
    },

    enterHandler() {
      this.onSelect(this.selection)
    },

    onSelect(index: number) {
      const item = this.items[index]
      if (item) {
        this.command(item)
      }
    },
  },
}
</script>
