<template>
  <div>
    <div class="bg-primary-dark flex h-12 w-full items-center gap-5 px-2 md:fixed md:top-0">
      <button @click="showLinks = !showLinks">
        <FontAwesomeIcon :icon="faBars" class="text-accent" />
      </button>
      <DynamicLogo class="h-10" :logo="type" />
      <slot name="header" />
      <button
        class="border-accent bg-primary-default h-10 w-10 rounded-sm border"
        @click="showSettings = true"
      >
        <FontAwesomeIcon :icon="faGears" />
      </button>
      <button
        class="border-accent bg-primary-default h-10 w-10 rounded-sm border"
        @click="logOut()"
      >
        <FontAwesomeIcon :icon="faRightFromBracket" />
      </button>
    </div>
    <div>
      <slot />
    </div>
    <SettingsPopUp v-if="showSettings" @close="showSettings = false"></SettingsPopUp>

    <div
      v-if="showLinks"
      class="border-accent bg- text-accent absolute top-12 left-0 flex w-screen flex-col gap-5 border-b p-5 pl-2 text-xl md:h-fit md:w-fit md:rounded-br md:border-0"
    >
      <RouterLink to="/">
        <div class="flex items-center gap-2">
          <FontAwesomeIcon :icon="faHouse" />
          <span>Home</span>
        </div>
      </RouterLink>
      <RouterLink :to="{ name: 'InventarWerk' }">
        <div class="flex items-center">
          <DynamicLogo logo="inventar" class="h-7" />
          <span>nventarWerk</span>
        </div>
      </RouterLink>
      <RouterLink :to="{ name: 'ZauberWerk' }">
        <div class="flex items-center">
          <DynamicLogo logo="zauber" class="h-7" />
          <span>auberWerk</span>
        </div>
      </RouterLink>
    </div>
  </div>
</template>

<script setup lang="ts">
import { faBars, faGears, faRightFromBracket } from '@fortawesome/free-solid-svg-icons'
import DynamicLogo from './DynamicLogo.vue'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import SettingsPopUp from '@/components/SettingsPopUp.vue'
import { ref, type PropType } from 'vue'
import { DatabaseHandler } from '@/store/DatabaseHandler.ts'
import { RouterLink } from 'vue-router'
import { faHouse } from '@fortawesome/free-regular-svg-icons'

defineProps({
  type: {
    type: String as PropType<'inventar' | 'zauber'>,
    required: true
  }
})

const showSettings = ref(false)
const showLinks = ref(false)

function logOut() {
  DatabaseHandler.getInstance()
    .logOut()
    .then(() => {
      window.location.reload()
    })
}
</script>
