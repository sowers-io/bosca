import { defineConfig } from 'vitest/config'

export default defineConfig({
  test: {
    include: ['**/*.spec.ts'],
    globals: true,
    environment: 'node',
    browser: {
      name: 'chrome',
    },
  },
})