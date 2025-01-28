<template>
  <div class="space-y-1 rounded bg-fuchsia-950 p-1">
    <h1 @click="collapsed = !collapsed">
      <FontAwesomeIcon :icon="collapsed ? faChevronRight : faChevronDown" class="h-4 w-4" />
      {{ group }}
    </h1>
    <div v-if="!collapsed" class="space-y-1 p-2">
      <PresetLine v-for="preset in presets" :key="preset.uuid" :preset="preset" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { faChevronDown, faChevronRight } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { ref } from 'vue'
import PresetLine from './PresetLine.vue'
import type { ItemPreset } from '@/model/ItemPreset'

defineProps({
  group: {
    type: String,
    required: true
  },
  presets: {
    type: Array<ItemPreset>,
    required: true
  }
})

const collapsed = ref(true)
</script>
