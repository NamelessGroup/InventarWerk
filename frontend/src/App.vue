<template>
  <div class="h-screen w-screen bg-slate-950 text-white">
    <div class="flex h-12 w-full items-center justify-end space-x-5 bg-fuchsia-950 px-2">
      <button class="h-10 w-10 rounded border border-amber-300 bg-fuchsia-900">
        <FontAwesomeIcon :icon="faGears" />
      </button>
      <button class="h-10 w-10 rounded border border-amber-300 bg-fuchsia-900">
        <FontAwesomeIcon :icon="faRightFromBracket" />
      </button>
    </div>
    <button class="absolute bottom-2 right-2 z-10 h-10 w-10 rounded border border-amber-300 bg-fuchsia-900">
        <FontAwesomeIcon :icon="faPlus" />
      </button>
    <div class="p-5">
      <InventoryContainer
        :inventory="store().getInvetory('123')"
      />
    </div>
  </div>
  <ErrorDisplay class="absolute z-20 bottom-0 w-screen" />
</template>

<script setup lang="ts">
import InventoryContainer from './components/InventoryContainer.vue'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faGears, faPlus, faRightFromBracket } from '@fortawesome/free-solid-svg-icons'
import { store } from './store';
import ErrorDisplay from './errorHandling/ErrorDisplay.vue';
</script>
