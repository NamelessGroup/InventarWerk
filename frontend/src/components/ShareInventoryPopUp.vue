<template>
<PopUp class="!m-0" @close="emit('close')">
  <h2 class="bold text-xl">Share Inventory</h2>
  <button class="rounded border border-amber-300 bg-fuchsia-900 outline-none p-1 w-full" @click="makePublic">Make public</button>
  <h3 class="bold text-lg mt-4">Read Access:</h3>
  <div>
    <ShareRow v-for="account in accountsWithReadAccess" :key="account.uuid" mode="remove" :name="account.name" @click="store().removeReadShare(inventory.uuid, account.uuid)" />
    <ShareRow v-for="account in accountCanHaveReadAccess" :key="account.uuid" :name="account.name" mode="add" @click="store().addReadShare(inventory.uuid, account.uuid)" />
  </div>
  <h3 class="bold text-lg mt-4">Write Access:</h3>
  <div>
    <ShareRow v-for="account in accountsWithWriteAccess" :key="account.uuid" :name="account.name" mode="remove" @click="store().removeWriteShare(inventory.uuid, account.uuid)" />
    <ShareRow v-for="account in accountCanHaveWriteAccess" :key="account.uuid" :name="account.name" mode="add" @click="store().addWriteShare(inventory.uuid, account.uuid)" />
  </div>
</PopUp>
</template>

<script setup lang="ts">
import { computed, type PropType } from 'vue';
import PopUp from './PopUp.vue';
import type { Inventory } from '@/model/Inventory';
import { store } from '@/store';
import ShareRow from './ShareRow.vue';

const props = defineProps({
  inventory: {
    type: Object as PropType<Inventory>,
    required: true
  }
})


const accounts = computed(() => store().accounts.map(account => {
  const hasRead = props.inventory.reader.includes(account.uuid);
  const hasWrite = props.inventory.writer.includes(account.uuid);
  return { ...account, hasRead, hasWrite }
}).filter(account => account.uuid != props.inventory.owner && !account.dm))

const accountsWithWriteAccess = computed(() => accounts.value.filter(account => account.hasWrite));
const accountsWithReadAccess = computed(() => accounts.value.filter(account => account.hasRead && !account.hasWrite));
const accountCanHaveWriteAccess = computed(() => accounts.value.filter(account => !account.hasWrite));
const accountCanHaveReadAccess = computed(() => accounts.value.filter(account => !account.hasRead && !account.hasWrite));

const emit = defineEmits(['close']);

async function makePublic() {
  await store().makePublic(props.inventory.uuid);
  emit('close')
}
</script>