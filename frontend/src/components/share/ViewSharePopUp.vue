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
        class="border-accent bg-primary-default w-full rounded-sm border p-1 outline-hidden"
        @click="leaveInventory()"
      >
        Leave inventory
      </button>
    </div>
  </PopUp>
</template>

<script setup lang="ts">
import type { Inventory } from '@/model/Inventory'
import { inventoryData, accountData } from '@/store'
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
    accountData().accounts.filter((account) => account.uuid === props.inventory.ownerUuid)[0] ?? {
      name: 'Unknown',
      avatar: null,
      dm: false,
      uuid: ''
    }
  )
})
const hasWriteAccess = computed(() => props.inventory.writer.includes(accountData().uuid))

const emit = defineEmits(['close'])

async function leaveInventory() {
  await inventoryData().removeReadShare(props.inventory.uuid, accountData().uuid)

  inventoryData().inventoryUuids = inventoryData().inventoryUuids.filter(
    (uuid) => uuid !== props.inventory.uuid
  )
  delete inventoryData().inventories[props.inventory.uuid]
  emit('close')
}
</script>
