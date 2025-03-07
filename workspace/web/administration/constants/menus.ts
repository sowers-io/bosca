import type { NavMenu, NavMenuItems } from '~/types/nav'

export const navMenu: NavMenu[] = [
  {
    heading: 'Contents',
    items: [
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
    heading: 'Profiles',
    items: [
      {
        title: 'Profiles',
        icon: 'i-lucide-contact',
        link: '/profiles',
      },
    ],
  },
]

export const navMenuBottom: NavMenuItems = []
