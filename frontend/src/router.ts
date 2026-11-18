import { createRouter, createWebHistory } from 'vue-router'
import HomePage from './views/HomePage.vue'
import InventarWerk from './views/InventarWerk.vue'
import ZauberWerk from './views/ZauberWerk.vue'
import inventarLogo from '@/assets/inventar/logo.png'
import zauberLogo from '@/assets/zauber/logo.png'

declare module 'vue-router' {
  interface RouteMeta {
    title?: string
    icon?: string
  }
}

const DEFAULT_TITLE = 'InventarWerk'
const DEFAULT_ICON = '/favicon.ico'

export const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'Home',
      component: HomePage
    },
    {
      path: '/inventory/',
      name: 'InventarWerk',
      component: InventarWerk,
      meta: { title: 'InventarWerk', icon: inventarLogo }
    },
    {
      path: '/zauberwerk/',
      name: 'ZauberWerk',
      component: ZauberWerk,
      meta: { title: 'ZauberWerk', icon: zauberLogo }
    }
  ]
})

router.afterEach((to) => {
  document.title = to.meta.title ?? DEFAULT_TITLE

  let link = document.querySelector<HTMLLinkElement>("link[rel~='icon']")
  if (!link) {
    link = document.createElement('link')
    link.rel = 'icon'
    document.head.appendChild(link)
  }
  link.href = to.meta.icon ?? DEFAULT_ICON
})
