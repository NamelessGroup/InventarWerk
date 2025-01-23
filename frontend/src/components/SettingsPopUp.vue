<template>
  <PopUp @close="emit('close')">
    <div>
      <h1 class="text-xl">Settings</h1>
      <div class="space-x-2">
        <input id="simplyGold" v-model="simplifyGold" type="checkbox" />
        <label for="simplyGold">Simplify gold into platinum:</label>
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
</script>