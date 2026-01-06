<template>
  <Transition
    name="collapse"
    :class="{ opacity: withOpacity }"
    @enter="updateFullHeight"
    @leave="updateFullHeight"
  >
    <slot />
  </Transition>
</template>

<script setup lang="ts">
import { ref } from 'vue'

defineProps({
  withOpacity: {
    type: Boolean,
    default: false
  },
  speed: {
    type: String,
    default: '0.2s'
  }
})

const fullHeight = ref('100vwh') // We have to guess a sensible initial value here, for the very first animation

function updateFullHeight(el: Element) {
  fullHeight.value = `${el.scrollHeight}px`
}
</script>

<style>
.collapse-enter-from,
.collapse-leave-to {
  max-height: 0px;
}

.collapse-enter-from.opacity,
.collapse-leave-to.opacity {
  opacity: 0;
}

.collapse-enter-active,
.collapse-leave-active {
  transition-property: max-height, opacity;
  transition-duration: v-bind(speed);
  transition-timing-function: ease, linear;
  overflow-y: hidden;
}

.collapse-enter-to,
.collapse-leave-from {
  max-height: v-bind(fullHeight);
}

.collapse-enter-to.opacity,
.collapse-leave-from.opacity {
  opacity: 100;
}
</style>
