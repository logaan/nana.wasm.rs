import { defineConfig } from 'vite'

export default defineConfig({
  build: {
    outDir: 'dist',
    emptyOutDir: true,
    rollupOptions: {
      input: 'index.html'
    },
    assetsDir: 'assets'
  },
  base: './',
  publicDir: 'public'
})
