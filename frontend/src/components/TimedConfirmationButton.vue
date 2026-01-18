<template>
  <button :class="{ fadeOut: confirmTimeRemaining > 0 }" @click="click">
    <slot v-if="confirmTimeRemaining <= 0" />
    <template v-else>
      <slot name="confirmation" />
      <template v-if="showRemainingTime"> {{ confirmTimeRemaining }}s </template>
    </template>
  </button>
</template>

<script setup lang="ts">
import { computed, onBeforeUnmount, ref } from 'vue'

const props = defineProps({
  confirmationTime: {
    type: Number,
    default: 5
  },
  showRemainingTime: {
    type: Boolean,
    default: true
  },
  skipConfirmation: {
    type: Function,
    default() {
      return false
    }
  }
})

const emit = defineEmits(['confirm'])

const confirmTimeRemaining = ref(-1)
const confirmTimer = ref<number | undefined>(undefined)

function click(e: Event) {
  e.stopPropagation()
  if (props.skipConfirmation()) {
    emit('confirm')
  } else if (confirmTimeRemaining.value < 0) {
    confirmTimeRemaining.value = props.confirmationTime
    confirmTimer.value = window.setInterval(timerTick, 1000)
  } else {
    clearInterval(confirmTimer.value)
    confirmTimer.value = undefined
    confirmTimeRemaining.value = -1
    emit('confirm')
  }
}

function timerTick() {
  confirmTimeRemaining.value -= 1
  if (confirmTimeRemaining.value <= 0) {
    clearInterval(confirmTimer.value)
    confirmTimer.value = undefined
    confirmTimeRemaining.value = -1
  }
}

const animationTime = computed(() => {
  return `${props.confirmationTime}s`
})

onBeforeUnmount(() => {
  clearInterval(confirmTimer.value)
})
</script>

<style scoped>
.fadeOut {
  border-style: solid;
  border-width: 2px;
  border-color: rgba(255, 0, 0, 0);
  animation: border-fade-out v-bind(animationTime) ease-in;
}

@keyframes border-fade-out {
  from {
    border-color: rgba(255, 0, 0, 1);
  }
  to {
    border-color: rgba(255, 0, 0, 0);
  }
}
</style>
