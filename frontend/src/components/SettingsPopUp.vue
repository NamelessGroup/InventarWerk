<template>
  <PopUp @close="emit('close')">
    <div class="flex min-h-full flex-col">
      <h1 class="text-xl">Settings</h1>
      <div class="grid grid-cols-1 gap-2 md:grid-cols-[auto_1fr]">
        <div class="space-x-2 md:col-span-2">
          <input id="simplyGold" v-model="simplifyGold" type="checkbox" />
          <label for="simplyGold">Simplify gold into platinum:</label>
        </div>

        <label for="fetchTime">Time between fetches (in seconds):</label>
        <input
          id="fetchTime"
          v-model="fetchTime"
          type="number"
          class="rounded-sm border border-amber-300 bg-fuchsia-900 px-1 outline-hidden"
        />
      </div>
      <div v-if="store().userIsDm" class="space-x-2 md:col-span-2">
        <input id="serverLock" v-model="serverLock" type="checkbox" />
        <label for="serverLock">Prohibit new users from joining</label>
      </div>

      <div class="flex-1"><!-- Placeholder --></div>
      <div class="ml-auto flex items-center gap-x-1 text-xs text-fuchsia-300">
        DM:
        <DiscordImage class="h-6" :user="dmAccount" />
        Hosted commit:
        <a
          class="underline"
          :href="`https://github.com/NamelessGroup/InventarWerk/tree/${version}`"
          >{{ version }}</a
        >
      </div>
    </div>
  </PopUp>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import PopUp from './PopUp.vue'
import { Settings } from '@/store/Settings'
import { store } from '@/store'
import { version } from '@/utils/version'
import DiscordImage from './DiscordImage.vue'

const emit = defineEmits(['close'])

const simplifyGold = computed({
  get: () => Settings.getInstance().breakDownGold,
  set: (value: boolean) => (Settings.getInstance().breakDownGold = value)
})

const fetchTime = computed({
  get: () => Settings.getInstance().timeBetweenFetches,
  set: (value: number) => (Settings.getInstance().timeBetweenFetches = value)
})

const serverLock = computed({
  get: () => store().isServerLocked,
  set: () => store().toggleLock()
})

const dmAccount = computed(() => {
  const dm = store().accounts.filter((a) => a.dm)
  return dm.length > 0
    ? dm[0]
    : {
        uuid: '123',
        name: 'DM',
        avatar: null,
        dm: true
      }
})
</script>
