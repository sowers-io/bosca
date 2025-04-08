export default defineAppConfig({
  ui: {
    colors: {
      primary: 'green',
      neutral: 'gray'
    },
  },
  uiPro: {
    footer: {
      slots: {
        root: 'border-t border-(--ui-border)',
        left: 'text-sm text-(--ui-text-muted)'
      }
    }
  },
  seo: {
    siteName: 'Bosca'
  },
  header: {
    title: '',
    to: '/',
    logo: {
      alt: '',
      light: '',
      dark: ''
    },
    search: true,
    colorMode: true,
    links: [{
      'icon': 'i-simple-icons-github',
      'to': 'https://github.com/sowers-io/bosca',
      'target': '_blank',
      'aria-label': 'GitHub'
    }]
  },
  footer: {
    credits: `Copyright Sowers, LLC © ${new Date().getFullYear()}`,
    colorMode: false,
    links: [{
      'icon': 'i-bosca-logo',
      'to': 'https://bosca.io',
      'target': '_blank',
      'aria-label': 'Bosca Website'
    }, {
      'icon': 'i-simple-icons-github',
      'to': 'https://github.com/sowers-io/bosca',
      'target': '_blank',
      'aria-label': 'Bosca on GitHub'
    }]
  },
  toc: {
    title: 'Table of Contents',
    bottom: {
      title: 'Community',
      edit: 'https://github.com/sowers-io/bosca/edit/main/docs/content',
      links: [{
        icon: 'i-lucide-star',
        label: 'Star on GitHub',
        to: 'https://github.com/sowers-io/bosca',
        target: '_blank'
      }]
    }
  }
})
