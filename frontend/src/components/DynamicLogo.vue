<template>
  <img :src="logoSrc" />
</template>

<script setup lang="ts">
import { computed, type PropType } from 'vue'
import iDefaultLogo from '@/assets/inventar/logo.png'
import iWhiteLogo from '@/assets/inventar/logo_white.png'
import iBlackLogo from '@/assets/inventar/logo_black.png'
import zDefaultLogo from '@/assets/zauber/logo.png'
import { ThemeManager } from '@/theme/ThemeManager'

type LogoFamily = 'inventar' | 'zauber'

const props = defineProps({
  logo: {
    type: String as PropType<LogoFamily>,
    required: true
  }
})

interface ILogo {
  default: string
  contrastDark: string
  contrastLight: string
}

const logoDict: Record<LogoFamily, ILogo> = {
  inventar: {
    default: iDefaultLogo,
    contrastDark: iWhiteLogo,
    contrastLight: iBlackLogo
  },
  zauber: {
    default: zDefaultLogo,
    contrastDark: zDefaultLogo,
    contrastLight: zDefaultLogo
  }
}

const logoSrc = computed(() => {
  switch (ThemeManager.getInstance().appliedTheme) {
    case 'contrastDark':
      return logoDict[props.logo].contrastDark
    case 'contrastLight':
      return logoDict[props.logo].contrastLight
    default:
      return logoDict[props.logo].default
  }
})
</script>
