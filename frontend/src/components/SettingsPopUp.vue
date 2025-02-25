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
      <div class="flex-1"><!-- Placeholder --></div>
      <div class="ml-auto text-xs text-fuchsia-300">
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
import { version } from '@/utils/version'

const emit = defineEmits(['close'])

const simplifyGold = computed({
  get: () => Settings.getInstance().breakDownGold,
  set: (value: boolean) => (Settings.getInstance().breakDownGold = value)
})

const fetchTime = computed({
  get: () => Settings.getInstance().timeBetweenFetches,
  set: (value: number) => (Settings.getInstance().timeBetweenFetches = value)
})
</script>
