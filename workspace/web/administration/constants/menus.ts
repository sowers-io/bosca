import type { NavMenu, NavMenuItems } from '~/types/nav'

export const navMenu: NavMenu[] = [
  {
    heading: 'Contents',
    items: [
      {
        title: 'Browse',
        icon: 'i-lucide-folder-tree',
        link: '/collections/browse',
      },
      {
        title: 'Collections',
        icon: 'i-lucide-folders',
        link: '/collections',
      },
      {
        title: 'Media',
        icon: 'i-lucide-tv-minimal-play',
        link: '/media',
      },
      {
        title: 'Content',
        icon: 'i-lucide-table-of-contents',
        link: '/content',
      },
      {
        title: 'Bible',
        icon: 'i-lucide-book-open-text',
        link: '/bible',
      },
    ],
  },
  {
    heading: 'Workflows',
    items: [
      {
        title: 'Traits',
        icon: 'i-lucide-boxes',
        link: '/workflows/traits',
      },
      {
        title: 'Workflows',
        icon: 'i-lucide-workflow',
        link: '/workflows',
      },
      {
        title: 'States',
        icon: 'i-lucide-waypoints',
        link: '/workflows/states',
      },
      {
        title: 'Prompts',
        icon: 'i-lucide-bot-message-square',
        link: '/workflows/prompts',
      },
      {
        title: 'Models',
        icon: 'i-lucide-package',
        link: '/workflows/models',
      },
      {
        title: 'Storage',
        icon: 'i-lucide-database',
        link: '/workflows/storage',
      },
      {
        title: 'Transitions',
        icon: 'i-lucide-arrow-left-right',
        link: '/workflows/transitions',
      },
    ],
  },
  {
    heading: 'AI',
    items: [
      {
        title: 'Chat',
        icon: 'i-lucide-message-square-text',
        link: '/chat',
      },
    ],
  },
  {
    heading: 'Principals & Profiles',
    items: [
      {
        title: 'Principals',
        icon: 'i-lucide-user-round-search',
        link: '/principals',
      },
      {
        title: 'Groups',
        icon: 'i-lucide-users',
        link: '/groups',
      },
      {
        title: 'Profiles',
        icon: 'i-lucide-contact',
        link: '/profiles',
      },
    ],
  },
]

export const navMenuBottom: NavMenuItems = []
