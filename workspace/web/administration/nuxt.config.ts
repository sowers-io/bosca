import { vite as vidstack } from 'vidstack/plugins'

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: false },

  devServer: {
    port: 3001,
  },

  nitro: {
    preset: 'denoServer',
  },

  modules: [
    '@nuxtjs/color-mode',
    '@nuxtjs/tailwindcss',
    '@vueuse/nuxt',
    'shadcn-nuxt',
    '@nuxt/icon',
  ],

  vue: {
    compilerOptions: {
      isCustomElement: (tag) => tag.startsWith('media-'),
    },
  },

  shadcn: {
    prefix: '',
    componentDir: './components/ui',
  },

  vite: {
    plugins: [vidstack()],
  },

  colorMode: {
    classSuffix: '',
  },

  routeRules: {
    '/components': { redirect: '/components/accordion' },
    '/settings': { redirect: '/settings/profile' },
  },

  runtimeConfig: {
    graphqlUrl: 'http://localhost:8000/graphql',
    graphqlWsUrl: 'ws://localhost:8000/ws',
    public: {
      domain: 'http://localhost',
      graphqlUrl: 'http://localhost:8000/graphql',
      graphqlWsUrl: 'ws://localhost:8000/ws',
    },
  },

  imports: {
    dirs: [
      './lib',
    ],
  },

  compatibilityDate: '2025-03-07',
})
