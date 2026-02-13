<template>
  <button ref="root" :class="{ fadeOut: confirmTimeRemaining > 0 }" @click="click">
      <slot v-if="confirmTimeRemaining <= 0" />
    <template v-else>
      <slot name="confirmation" />
      <template v-if="showRemainingTime"> {{ confirmTimeRemaining }}s </template>
    </template>

    <div ref="styleHolder"></div>

  </button>
</template>

<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, ref, useTemplateRef } from 'vue'

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
    // We need to wait for the DOM to update before we can calculate the animation as they need the resized button
    nextTick(() => setAnimation())
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

const root = useTemplateRef('root')
const styleHolder = useTemplateRef('styleHolder')
const bgColor = computed(() => {
  if (!root.value) {
    return 'transparent'
  }
  const style = getComputedStyle(root.value!)
  return style.backgroundColor
})
const borderRadius = computed(() => {
  if (!root.value) {
    return '0px'
  }
  const style = getComputedStyle(root.value!)
  const childBorder = style.borderRadius.substring(0, style.borderRadius.length - 2)
  return `${Math.max(0, parseFloat(childBorder) - 2)}px`
})

const animationTime = computed(() => {
  return `${props.confirmationTime}s`
})
function setAnimation() {
  if (!root.value || !styleHolder.value) {
    return
  }
  const width = root.value.offsetWidth
  const height = root.value.offsetHeight
  const totalLength = 2 * width + 2 * height
  const angles: number[] = []

  for (let i = 1; i < 100; i++) {
    const progress = i / 100
    const length = totalLength * progress
    let x = -1;
    let y = -1;
    if (length < width / 2) {
      x = -length
      y = height / 2
    } else if (length < width / 2 + height) {
      x = - width / 2
      y = height / 2 - (length - width / 2)
    } else if (length < 1.5 * width + height) {
      x = - width / 2 + (length - width / 2 - height)
      y = - height / 2
    } else if (length < 1.5 * width + 2 * height) {
      x = width / 2
      y = - height / 2 + (length - 1.5 * width - height)
    } else {
      x = width / 2 - (length - 1.5 * width - 2 * height)
      y = height / 2
    }
    angles.push((Math.atan2(x,y) * 180 / Math.PI + 360) % 360)
  }
  const keyframes = `
  <style>
  @keyframes border-progress-bar {
    from {
      --border-progress-fill: 100%;
    }
    ${angles.map((angle, index) => {
      return `${index + 1}% { --border-progress-fill: ${angle * 100 / 360}%; }`
    }).join('\n')}
    to {
      --border-progress-fill: 0%;
    }
  }
  </style>
  `
  styleHolder.value.innerHTML = keyframes
}
onBeforeUnmount(() => {
  clearInterval(confirmTimer.value)
})
</script>

<style scoped>
@property --border-progress-fill {
  syntax: "<percentage>";
  inherits: false;
  initial-value: 100%;
}

.fadeOut {
  position: relative;
  background-image: conic-gradient(var(--color-red-500), var(--color-red-500) var(--border-progress-fill), transparent var(--border-progress-fill));
  z-index: 0;
  animation: border-progress-bar v-bind(animationTime) linear forwards;
  border: none;
}

.fadeOut::before {
  content: '';
  position: absolute;
  inset: 2px;
  border-radius: v-bind(borderRadius);
  border: inherit;
  background-color: v-bind(bgColor);
  z-index: -1;
}
</style>
