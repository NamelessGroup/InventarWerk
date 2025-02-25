<template>
  <div
    class="absolute bottom-0 left-0 right-0 top-0 z-30 flex items-center justify-center p-4 backdrop-blur-[1px]"
    @click="
      ($event) => {
        $event.stopPropagation()
        emit('close')
      }
    "
  >
    <div
      class="z-40 h-full w-full overflow-visible rounded-sm border border-amber-300 bg-slate-900 p-5 blur-0 md:h-[60vh] md:w-[80vw]"
      :class="popupClass"
      @click="($event) => $event.stopPropagation()"
    >
      <button
        class="absolute -right-2 -top-2 z-50 flex h-7 w-7 items-center rounded-full border border-amber-300 bg-slate-900 text-center text-xl"
        @click="emit('close')"
      >
        <p class="w-full text-center">
          <FontAwesomeIcon :icon="faTimes" />
        </p>
      </button>
      <div class="h-full overflow-scroll">
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
