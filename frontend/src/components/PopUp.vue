<template>
  <div
    class="absolute bottom-4 left-4 right-4 top-4 z-40 overflow-hidden rounded border border-amber-300 bg-slate-900 p-5"
  >
    <button class="absolute right-1 top-0 text-2xl" @click="emit('close')">
      <FontAwesomeIcon :icon="faTimes" />
    </button>
    <slot></slot>
  </div>
</template>

<script setup lang="ts">
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faTimes } from '@fortawesome/free-solid-svg-icons'

const emit = defineEmits(['close'])
</script>
