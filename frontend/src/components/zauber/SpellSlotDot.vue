<template>
  <span
    class="border-accent inline-flex h-6 w-6 items-center justify-center rounded-full border"
    :class="filled ? 'bg-primary-dark text-text' : 'bg-primary-light text-transparent'"
  >
    <svg
      viewBox="0 0 24 24"
      class="h-full w-full p-1"
      fill="none"
      stroke="currentColor"
      stroke-width="6"
      stroke-linecap="round"
    >
      <path d="M0 0L24 24M24 0L0 24" />
    </svg>
  </span>
</template>

<script setup lang="ts">
defineProps({
  filled: { type: Boolean, default: false }
})
</script>
