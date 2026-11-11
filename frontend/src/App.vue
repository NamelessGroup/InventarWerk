<template>
  <div class="bg-background text-text flex min-h-screen w-screen flex-col">
    <RouterView class="flex-1" />
    <a
      :href="`https://github.com/NamelessGroup/InventarWerk/tree/${version}`"
      class="text-opacity-30 text-primary-text p-1 text-xs underline"
      >{{ version }}</a
    >

    <ErrorDisplay class="absolute bottom-0 z-50 w-screen" />
  </div>
</template>

<script setup lang="ts">
import { accountData } from './store'
import ErrorDisplay from './errorHandling/ErrorDisplay.vue'
import { version } from './utils/version'
import { RouterView } from 'vue-router'
import { router } from './router.ts'
import { DB } from './store/db/index.ts'

checkLogIn()

function checkLogIn() {
  DB.account.isLoggedIn().then((res) => {
    accountData().isLoggedIn = res
    if (res) {
      DB.initialize()
    } else if (router.currentRoute.value.name !== 'Home') {
      router.push('/')
    }
  })
}
</script>
