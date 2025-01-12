<template>
  <div class="absolute bottom-4 right-4 top-4 left-4 z-40 p-5 rounded border border-amber-300 bg-slate-900"> 
    <button class="absolute top-0 right-1 text-2xl" @click="emit('close')">
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