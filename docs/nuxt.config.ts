// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: [
    '@nuxt/eslint',
    '@nuxt/image',
    '@nuxt/ui-pro',
    '@nuxt/content',
    'nuxt-og-image',
    'nuxt-llms'
  ],

  devtools: {
    enabled: true
  },

  css: ['~/assets/css/main.css'],

  content: {
    build: {
      markdown: {
        toc: {
          searchDepth: 2
        }
      }
    }
  },

  future: {
    compatibilityVersion: 4
  },

  compatibilityDate: '2024-07-11',

  nitro: {
    prerender: {
      routes: [
        '/'
      ],
      crawlLinks: true
    }
  },

  eslint: {
    config: {
      stylistic: {
        commaDangle: 'never',
        braceStyle: '1tbs'
      }
    }
  },

  icon: {
    provider: 'iconify',
    customCollections: [{
      prefix: 'bosca',
      dir: './assets/icons'
    }]
  },

  llms: {
    domain: 'https://bosca.io',
    title: 'Bosca',
    description: 'Bosca is an AI-powered Content Management, Analytics, and Personalization platform built to help organizations unlock the full potential of their content strategies.',
    full: {
      title: 'Bosca',
      description: 'Bosca is an AI-powered Content Management, Analytics, and Personalization platform built to help organizations unlock the full potential of their content strategies.'
    },
    sections: [
      {
        title: 'Getting Started',
        contentCollection: 'docs',
        contentFilters: [
          { field: 'path', operator: 'LIKE', value: '/getting-started%' }
        ]
      }
    ]
  }
})
