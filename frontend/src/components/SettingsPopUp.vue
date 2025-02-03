<template>
  <PopUp @close="emit('close')">
    <div class="min-h-full flex flex-col">
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
          class="rounded border border-amber-300 bg-fuchsia-900 px-1 outline-none"
        />
      </div>
      <div class="flex-1"><!-- Placeholder --></div>
      <div class="text-fuchsia-300 text-xs ml-auto">Hosted commit: {{ version }}</div>
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
