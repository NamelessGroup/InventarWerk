<template>
  <div>
    <PageBase class="h-full w-full" type="zauber">
      <template #header>
        <div class="flex-1"></div>
      </template>
      <template #default><div></div></template>
    </PageBase>
  </div>
</template>

<script setup lang="ts">
import PageBase from '@/components/PageBase.vue'
import { ThemeManager } from '@/theme/ThemeManager'

ThemeManager.getInstance().pageTheme = 'zauber'
</script>
