<template>
  <div class="mt-40 flex h-full w-full items-start justify-center gap-5">
    <div v-if="!store().isLoggedIn" class="grid max-w-[80vw] grid-cols-2 gap-5">
      <DynamicLogo logo="inventar" />
      <DynamicLogo logo="zauber" />

      <button
        class="border-accent bg-primary-default col-span-2 row-start-2 h-12 cursor-pointer rounded-sm border"
        @click="logIn()"
      >
        Login
      </button>
      <p class="text-accent/50 col-span-2 row-start-3 text-center">
        This website uses cookies. No you can not reject.
      </p>
    </div>

    <div v-else class="text-accent flex flex-col items-center gap-5 text-5xl md:text-9xl">
      <RouterLink
        :to="{ name: 'InventarWerk' }"
        class="border-accent bg-primary-default col-span-2 row-start-2 w-full rounded-sm border px-5 py-2"
      >
        <div class="flex items-center justify-center">
          <DynamicLogo logo="inventar" class="h-12 md:h-32" />
          <span>nventarWerk</span>
        </div>
      </RouterLink>

      <RouterLink
        :to="{ name: 'ZauberWerk' }"
        class="border-accent bg-primary-default col-span-2 row-start-2 w-full rounded-sm border px-5 py-2"
      >
        <div class="flex items-center justify-center">
          <DynamicLogo logo="zauber" class="h-12 md:h-32" />
          <span>auberWerk</span>
        </div>
      </RouterLink>

      <button
        class="border-accent bg-primary-default col-span-2 row-start-2 flex w-fit items-center gap-2 rounded-sm border px-5 py-2 text-xl"
        @click="logOut()"
      >
        <FontAwesomeIcon :icon="faRightFromBracket" />
        <span>LogOut</span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import DynamicLogo from '@/components/DynamicLogo.vue'
import { store } from '@/store'
import { DatabaseHandler } from '@/store/DatabaseHandler'
import { ThemeManager } from '@/theme/ThemeManager'
import { faRightFromBracket } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

ThemeManager.getInstance().pageTheme = 'home'

function logIn() {
  window.location.href = DatabaseHandler.getInstance().getLogInUrl()
}

function logOut() {
  DatabaseHandler.getInstance()
    .logOut()
    .then(() => {
      window.location.reload()
    })
}
</script>
