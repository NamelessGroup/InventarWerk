<template>
  <PopUp @close="emit('close')">
    <div>
      <p class="flex items-center gap-2">
        Shared by <DiscordImage :user="creator" class="h-8" /> {{ creator.name }}
      </p>
      <p v-if="hasWriteAccess">
        Write access: You can add and remove items and money from this inventory.
      </p>
      <p v-else>Read access: You can view items and money from this inventory, but not edit it.</p>
      <button
        class="w-full rounded-sm border border-amber-300 bg-fuchsia-900 p-1 outline-hidden"
        @click="leaveInventory()"
      >
        Leave inventory
      </button>
    </div>
  </PopUp>
</template>

<script setup lang="ts">
import type { Inventory } from '@/model/Inventory'
import { store } from '@/store'
import { computed, type PropType } from 'vue'
import DiscordImage from '../DiscordImage.vue'
import PopUp from '../PopUp.vue'

const props = defineProps({
  inventory: {
    type: Object as PropType<Inventory>,
    required: true
  }
})

const creator = computed(() => {
  return (
    store().accounts.filter((account) => account.uuid === props.inventory.owner)[0] ?? {
      name: 'Unknown',
      avatar: null,
      dm: false,
      uuid: ''
    }
  )
})
const hasWriteAccess = computed(() => props.inventory.writer.includes(store().uuid))

const emit = defineEmits(['close'])

async function leaveInventory() {
  if (hasWriteAccess.value) {
    await store().removeWriteShare(props.inventory.uuid, store().uuid)
  }
  store().removeReadShare(props.inventory.uuid, store().uuid)

  store().inventoryUuids = store().inventoryUuids.filter((uuid) => uuid !== props.inventory.uuid)
  delete store().inventories[props.inventory.uuid]
  emit('close')
}
</script>
