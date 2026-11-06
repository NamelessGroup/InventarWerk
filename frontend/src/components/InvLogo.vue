<template>
  <img :src="logo" />
</template>

<script setup lang="ts">
import { computed } from 'vue'
import standardLogo from '@/assets/logo.png'
import whiteLogo from '@/assets/logo_white.png'
import blackLogo from '@/assets/logo_black.png'
import { ThemeManager } from '@/theme/ThemeManager'

const logo = computed(() => {
  switch (ThemeManager.getInstance().appliedTheme) {
    case 'contrastDark':
      return whiteLogo
    case 'contrastLight':
      return blackLogo
    default:
      return standardLogo
  }
})
</script>
