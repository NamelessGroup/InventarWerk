<template>
  <PopUp @close="emit('close')">
    <div class="flex items-center justify-center">
      <div
        class="grid-rows-[repeat(5, auto)] grid h-fit w-fit gap-5 md:grid-cols-2 md:grid-rows-[repeat(3,auto)]"
      >
        <span>spells-XX.json</span>
      </div>
    </div>
  </PopUp>
</template>

<script setup lang="ts">
import PopUp from '../PopUp.vue'

const emit = defineEmits(['close'])
</script>
