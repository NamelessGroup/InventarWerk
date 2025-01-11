<template>
  <div class="absolute bottom-2 right-2 top-2 left-2 z-50 p-5 pt-10 rounded border border-amber-300 bg-slate-900"> 
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