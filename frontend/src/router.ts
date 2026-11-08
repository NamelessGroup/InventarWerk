import { createRouter, createWebHistory } from 'vue-router'
import HomePage from './views/HomePage.vue'
import InventarWerk from './views/InventarWerk.vue'
import ZauberWerk from './views/ZauberWerk.vue'

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
      component: InventarWerk
    },
    {
      path: '/zauberwerk/',
      name: 'ZauberWerk',
      component: ZauberWerk
    }
  ]
})
