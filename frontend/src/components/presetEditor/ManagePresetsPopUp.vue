<template>
  <PopUp @close="emit('close')">
    <div class="max-h-full space-y-2 overflow-auto">
      <input
        v-model="searchFieldContent"
        type="text"
        placeholder="Search"
        class="w-full rounded border border-amber-300 bg-fuchsia-950 p-1 outline-none"
      />
      <PresetGroup
        v-for="group in allGroups"
        :key="group"
        :group="group"
        :presets="groupedPresets[group]"
      />
    </div>
  </PopUp>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import PopUp from '../PopUp.vue'
import { store } from '@/store'
import PresetGroup from './PresetGroup.vue'
import type { ItemPreset } from '@/model/ItemPreset'

const emit = defineEmits(['close'])
const searchFieldContent = ref('')

const allPresets = computed(() => {
  return store().itemPresets.sort((a, b) => {
    if (a.itemType == b.itemType) {
      return a.name.localeCompare(b.name)
    }
    if (a.itemType == 'Other') {
      return 1
    }
    if (b.itemType == 'Other') {
      return -1
    }
    return a.itemType.localeCompare(b.itemType)
  })
})

const filteredPresets = computed(() => {
  return allPresets.value.filter((p) =>
    p.name.toLowerCase().includes(searchFieldContent.value.toLowerCase())
  )
})

const allGroups = computed(() => {
  return Array.from(new Set(filteredPresets.value.map((p) => p.itemType)))
})

const groupedPresets = computed(() => {
  const grouped = allGroups.value.map((group) => {
    return {
      group: group,
      presets: allPresets.value.filter((p) => p.itemType === group)
    }
  })
  const result: Record<string, ItemPreset[]> = {}
  grouped.forEach((group) => {
    result[group.group] = group.presets
  })

  return result
})
</script>
