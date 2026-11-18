<template>
  <div class="grid grid-cols-1 gap-5 md:grid-cols-2 lg:grid-cols-3">
    <template v-if="Settings.getInstance().strictInventoryGrid">
      <slot v-for="item in containers" :key="item.uuid" :item="item" />
    </template>
    <template v-else>
      <div class="flex flex-col gap-5 md:hidden">
        <slot v-for="item in containers" :key="item.uuid" :item="item" />
      </div>
      <div v-for="i in 2" :key="i" class="hidden flex-col gap-5 md:flex lg:hidden">
        <slot v-for="item in getNthOfKContainers(i - 1, 2)" :key="item.uuid" :item="item" />
      </div>
      <div v-for="i in 3" :key="i" class="hidden flex-col gap-5 lg:flex">
        <slot v-for="item in getNthOfKContainers(i - 1, 3)" :key="item.uuid" :item="item" />
      </div>
    </template>
  </div>
</template>

<script setup lang="ts" generic="T extends { uuid: string }">
import { Settings } from '@/store/Settings'

const props = defineProps({
  containers: {
    type: Array<T>,
    required: true
  }
})

function getNthOfKContainers(n: number, k: number) {
  return props.containers.filter((_, idx) => idx % k === n)
}
</script>
