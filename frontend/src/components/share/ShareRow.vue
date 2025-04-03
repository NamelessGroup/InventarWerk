<template>
  <div class="flex w-full cursor-pointer items-center gap-2">
    <DiscordImage :user="account" class="h-8" />
    <span>{{ account.name }}</span>
    <select
      v-model="val"
      class="flex-1 rounded-sm border border-amber-300 bg-fuchsia-900 px-1 outline-hidden"
      @change="emit('update', val)"
    >
      <option value="none">No access</option>
      <option value="read">Read access</option>
      <option value="write">Write access</option>
    </select>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, type PropType } from 'vue'
import DiscordImage from '../DiscordImage.vue'
import type { Account } from '@/model/Account'

const props = defineProps({
  account: {
    type: Object as PropType<Account>,
    required: true
  },
  value: {
    type: String as PropType<Access>,
    required: true
  }
})

const emit = defineEmits(['update'])

type Access = 'read' | 'write' | 'none'

const val = ref(props.value)

watch(
  () => props.value,
  (value) => {
    val.value = value
  }
)
</script>
