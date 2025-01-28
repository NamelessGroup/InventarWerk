<template>
  <PopUp @close="emit('close')">
    <div>
      <h1 class="text-xl">Settings</h1>
      <div class="grid grid-cols-1 gap-2 md:grid-cols-[auto_1fr]">
        <div class="md:col-span-2 space-x-2">
          <input id="simplyGold" v-model="simplifyGold" type="checkbox" />
          <label for="simplyGold">Simplify gold into platinum:</label>
        </div>

        <label for="fetchTime">Time between fetches (in seconds):</label>
        <input id="fetchTime" v-model="fetchTime" type="number" class="rounded border border-amber-300 bg-fuchsia-900 outline-none px-1" />
      </div>
    </div>
  </PopUp>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import PopUp from './PopUp.vue'
import { Settings } from '@/store/Settings';

const emit = defineEmits(['close'])

const simplifyGold = computed({
  get: () => Settings.getInstance().breakDownGold,
  set: (value: boolean) => Settings.getInstance().breakDownGold = value
})

const fetchTime = computed({
  get: () => Settings.getInstance().timeBetweenFetches,
  set: (value: number) => Settings.getInstance().timeBetweenFetches = value
})
</script>