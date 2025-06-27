<template>
  <PopUp class="m-0!" @close="emit('close')">
    <div class="space-y-2">
      <h2 class="bold text-xl">Share Inventory</h2>
      <button
        class="w-full rounded-sm border border-amber-300 bg-fuchsia-900 p-1 outline-hidden"
        @click="makePublic"
      >
        Make public
      </button>
      <div class="space-y-2">
        <ShareRow
          v-for="account in accounts"
          :key="account.uuid"
          :value="account.accessType"
          :account="account"
          @update="(val) => updateShare(val, account.uuid)"
        />
      </div>
    </div>
  </PopUp>
</template>

<script setup lang="ts">
import { computed, type PropType } from 'vue'
import PopUp from '../PopUp.vue'
import type { Inventory } from '@/model/Inventory'
import { store } from '@/store'
import ShareRow from './ShareRow.vue'
import type { Account } from '@/model/Account'

const props = defineProps({
  inventory: {
    type: Object as PropType<Inventory>,
    required: true
  }
})
type Access = 'read' | 'write' | 'none'

const accounts = computed(() =>
  store()
    .accounts.map((account) => {
      const hasRead = props.inventory.reader.includes(account.uuid)
      const hasWrite = props.inventory.writer.includes(account.uuid)
      return {
        ...account,
        accessType: hasWrite ? 'write' : hasRead ? 'read' : 'none'
      } as Account & { accessType: Access }
    })
    .filter((account) => account.uuid != props.inventory.ownerUuid && !account.dm)
)

const emit = defineEmits(['close'])

async function makePublic() {
  await store().makePublic(props.inventory.uuid)
  emit('close')
}

async function updateShare(val: Access, uuid: string) {
  const user = accounts.value.find((account) => account.uuid === uuid)
  if (!user) return
  if (val === 'none') {
    if (user.accessType == 'read') {
      store().removeReadShare(props.inventory.uuid, uuid)
    } else if (user.accessType == 'write') {
      await store().removeWriteShare(props.inventory.uuid, uuid)
      store().removeReadShare(props.inventory.uuid, uuid)
    }
  } else if (val === 'read') {
    if (user.accessType === 'write') {
      await store().removeWriteShare(props.inventory.uuid, uuid)
    } else {
      store().addReadShare(props.inventory.uuid, uuid)
    }
  } else if (val === 'write') {
    store().addWriteShare(props.inventory.uuid, uuid)
  }
  user.accessType = val
}
</script>
