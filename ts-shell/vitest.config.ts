import { defineConfig } from 'vitest/config'

export default defineConfig({
  test: {
    exclude: ['ui/**', '**/ui/**', 'node_modules/**', 'dist/**'],
  },
})
