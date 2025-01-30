<template>
  <img :src="discordImage(user)" class="rounded-full" />
</template>

<script setup lang="ts">
import type { Account } from '@/model/Account'
import type { PropType } from 'vue'
import { discordImage } from '@/utils/discordImage'

defineProps({
  user: {
    type: Object as PropType<Account>,
    required: true
  }
})
</script>
