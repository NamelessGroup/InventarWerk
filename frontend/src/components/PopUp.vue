<template>
  <div class="absolute top-0 left-0 right-0 bottom-0 z-30 backdrop-blur-[1px] p-4 flex justify-center items-center" @click="($event) => {$event.stopPropagation();emit('close');}">
    <div
    class="w-full h-full z-40 overflow-visible rounded border border-amber-300 bg-slate-900 p-5 blur-0 md:h-[60vh] md:w-[80vw]"
    :class="popupClass"
    @click="($event) => $event.stopPropagation()"
  >
    <button class="absolute -right-2 -top-2 text-xl h-7 w-7 flex text-center items-center bg-slate-900 border-amber-300 z-50 rounded-full border" @click="emit('close')">
      <p class="w-full text-center">
        <FontAwesomeIcon :icon="faTimes" /></p>
    </button>
    <div class="overflow-scroll h-full">
      <slot></slot>
    </div>
  </div>
  </div>
</template>

<script setup lang="ts">
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faTimes } from '@fortawesome/free-solid-svg-icons'

defineProps({
  popupClass: {
    type: String,
    default: ''
  }
})

const emit = defineEmits(['close'])
</script>
