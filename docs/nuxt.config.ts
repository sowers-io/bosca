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
      crawlLinks: true,
      autoSubfolderIndex: true
    },
    hooks: {
      // Use 'prerender:routes' hook to dynamically add routes
      async 'prerender:routes'(routes) {
        routes.add('/')
        routes.add('/getting-started')
        routes.add('/content')
        routes.add('/content/metadata')
        routes.add('/content/collections')
        routes.add('/workflows')
        routes.add('/architecture')
        // try {
        //   // Fetch your dynamic data (e.g., post slugs or IDs) from your API
        //   const posts = await $fetch('https://your-api.com/posts?fields=slug'); // Adjust API endpoint
        //
        //   // Add each post route to the routes Set
        //   for (const post of posts) {
        //     routes.add(`/posts/${post.slug}`); // Use routes.add() inside the hook
        //   }
        //   console.log('Dynamically added routes:', routes);
        // } catch (error) {
        //   console.error('Failed to fetch dynamic routes:', error);
        //   // Optionally, throw an error to fail the build if routes are critical
        //   // throw new Error('Could not fetch dynamic routes for prerendering.');
        // }
      }
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
