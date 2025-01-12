<template>
  <div class="w-full cursor-pointer"  :class="mode == 'add' ? 'text-green-500' : 'text-red-500'">
    <FontAwesomeIcon :icon="mode == 'add' ? faUserPlus : faUserMinus" />
    {{ name }}
  </div>
</template>

<script setup lang="ts">
import { faUserPlus, faUserMinus } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import type { PropType } from 'vue';

defineProps({
  name: {
    type: String,
    required: true
  },
  mode: {
    type: String as PropType<'add' | 'remove'>,
    required: true
  }
})

</script>