<template>
  <button @click="click">
    <slot v-if="confirmTimeRemaining < 0" />
    <template v-else>
      <slot name="confirmation" />
      <template v-if="showRemainingTime">
        {{ confirmTimeRemaining }}
      </template>
    </template>
  </button>
</template>

<script setup lang="ts">
import { onBeforeUnmount, ref } from 'vue'

const props = defineProps({
  confirmationTime: {
    type: Number,
    default: 5
  },
  showRemainingTime: {
    type: Boolean,
    default: true
  }
})

const emit = defineEmits(['click'])

const confirmTimeRemaining = ref(-1)
const confirmTimer = ref<number | undefined>(undefined)

function click(e: Event) {
  e.stopPropagation()
  if (confirmTimeRemaining.value < 0) {
    confirmTimeRemaining.value = props.confirmationTime
    confirmTimer.value = setInterval(timerTick, 1000)
  } else {
    clearInterval(confirmTimer.value)
    confirmTimer.value = undefined
    confirmTimeRemaining.value = -1
    emit('click')
  }
}

function timerTick() {
  confirmTimeRemaining.value -= 1
  if (confirmTimeRemaining.value < 0) {
    clearInterval(confirmTimer.value)
    confirmTimer.value = undefined
    confirmTimeRemaining.value = -1
  }
}

onBeforeUnmount(() => {
  clearInterval(confirmTimer.value)
})
</script>
