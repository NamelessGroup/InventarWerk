<template>
  <Transition appear name="popup" @after-leave="transitionFinished">
    <div
      v-show="showPopup"
      class="fixed top-0 right-0 bottom-0 left-0 z-30 flex items-center justify-center bg-black/20 p-4 backdrop-blur-[2px]"
      @click="
        ($event) => {
          $event.stopPropagation()
          close()
        }
      "
    >
      <div
        class="popup-card blur-0 relative z-40 h-full w-full overflow-visible rounded-sm border border-amber-300 bg-slate-900 p-5 md:h-[60vh] md:w-[80vw]"
        :class="popupClass"
        @click="($event) => $event.stopPropagation()"
      >
        <button
          class="absolute -top-2 -right-2 z-50 flex h-7 w-7 items-center rounded-full border border-amber-300 bg-slate-900 text-center text-xl"
          @click="close()"
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
  </Transition>
</template>

<script setup lang="ts">
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faTimes } from '@fortawesome/free-solid-svg-icons'
import { ref } from 'vue'

defineProps({
  popupClass: {
    type: String,
    default: ''
  }
})

const showPopup = ref(true)

const emit = defineEmits(['close'])

function close() {
  showPopup.value = false
}

function transitionFinished() {
  emit('close')
}
</script>

<style>
.popup-enter-from,
.popup-leave-to {
  opacity: 0;
}

.popup-enter-from .popup-card,
.popup-leave-to .popup-card {
  transform: translateY(-20px);
}

.popup-enter-to,
.popup-leave-from {
  opacity: 100;
}

.popup-enter-to .popup-card,
.popup-leave-from .popup-card {
  transform: translateY(0px);
}

.popup-enter-active,
.popup-leave-active {
  transition-property: opacity;
  transition-duration: 0.2s;
  transition-timing-function: linear;
}

.popup-enter-active .popup-card,
.popup-leave-active .popup-card {
  transition-property: transform;
  transition-duration: 0.2s;
  transition-timing-function: ease;
}
</style>
