<template>
  <PopUp @close="emit('close')">
    <div class="flex flex-col gap-2">
      <div>Filter coming soon</div>
      <div class="flex flex-1 flex-col gap-2 overflow-auto">
        <SpellDisplay
          v-for="preset in presets"
          :key="preset.uuid"
          :spell="preset"
          :can-delete="accountData().userIsDm"
          @delete="zauberData().deletePreset(preset.uuid)"
        />
      </div>
    </div>
  </PopUp>
</template>

<script setup lang="ts">
import { accountData, zauberData } from '@/store'
import { computed } from 'vue'
import PopUp from '../PopUp.vue'
import SpellDisplay from './SpellDisplay.vue'

const emit = defineEmits(['close'])

const presets = computed(() => {
  const filtered = zauberData().spellPresetList
  const mapped = filtered.map((s) => ({ ...s, sorting: -1 }))
  return mapped.sort((a, b) => {
    const levelDiff = b.level - a.level
    if (levelDiff !== 0) return levelDiff
    return b.name.localeCompare(a.name)
  })
})
</script>
