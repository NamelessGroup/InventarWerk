<template>
  <div class="space-y-2 rounded border-2 border-amber-300 bg-fuchsia-950 p-2 overflow-hidden">
    <div class="flex gap-2 align-middle">
      <div class="bold text-xl bg-transparent outline-none border-none pr-5" >{{ inventory.name }}</div>
      <div>({{ inventory.items.map(i => i.weight).reduce((a,b) => a+b, 0) }} lbs.)</div>
      <div class="flex-1"><!-- Spacer --></div>
      <button v-if="inventory.owner === store().uuid" class="rounded border border-amber-300 bg-fuchsia-900 w-7 h-7">
        <FontAwesomeIcon :icon="faShare" @click="showSharePopup = true" />
      </button>
      <button v-if="inventory.owner === store().uuid" class="rounded border border-amber-300 bg-fuchsia-900 w-7 h-7" @click="deleteInventory">
        <FontAwesomeIcon :icon="faTrashCan" class="text-red-300" />
      </button>
    </div>
    <div class="grid max-w-full grid-cols-4 gap-x-2 overflow-auto">
      <NumericInput 
        v-for="[k, i] of moneyOptions"
        :key="k"
        v-model="moneyFieldValues[k]"
        class="row-start-1 h-10 rounded border border-amber-300 bg-fuchsia-900 outline-none px-1"
        :class="`col-start-${i}`"
        @update="v => updateMoney(v, k)"
      />      
      <span
        v-for="[k, i, l] of moneyOptions"
        :key="k + 'l'"
        :class="`col-start-${i}`"
        class="row-start-2 text-center text-sm text-amber-200"
        >{{ l }}</span
      >
    </div>

    <div class="space-y-2">
      <ItemRowDisplay v-for="item in inventory.items" :key="item.presetReference" :item="item" :inventory-uuid="inventory.uuid" />
    </div>

    <button class="h-10 w-full rounded bg-fuchsia-900 text-center" @click="showAddItemPopup = true">+ Add item</button>
  </div>
  <ShareInventoryPopUp v-if="showSharePopup" :inventory="inventory" @close="showSharePopup = false" />
  <AddItemPopUp v-if="showAddItemPopup" :inventory-uuid="inventory.uuid" @close="showAddItemPopup = false" />
</template>

<script setup lang="ts">
import { ref, watch, type PropType } from 'vue'
import type { Inventory } from '../model/Inventory'
import ItemRowDisplay from './ItemRowDisplay.vue'
import type { MoneyFields } from '@/utils/moneyMath';
import { store } from '@/store';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import { faShare, faTrashCan } from '@fortawesome/free-solid-svg-icons';
import AddItemPopUp from './AddItemPopUp.vue';
import ShareInventoryPopUp from './ShareInventoryPopUp.vue';
import NumericInput from './NumericInput.vue';

const props = defineProps({
  inventory: {
    type: Object as PropType<Inventory>,
    required: true
  }
})

const inventoryName = ref(props.inventory.name)
const nameInput = ref<HTMLInputElement | null>(null)
const showSharePopup = ref(false)
const showAddItemPopup = ref(false)

// eslint-disable-next-line @typescript-eslint/no-unused-vars
function editName() {
  if (nameInput.value) {
    nameInput.value.focus()
  }
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
function updateName() {
  if (inventoryName.value.length == 0) {
    return
  }
  if (inventoryName.value == props.inventory.name) {
    return
  }
}

function deleteInventory() {
  const result = confirm('This will delete the inventory forever')
  if (!result) {
    return
  }
  store().deleteInventory(props.inventory.uuid)
}

const moneyFieldValues = ref({
  platinum: props.inventory.money.platinum,
  gold: props.inventory.money.gold,
  silver: props.inventory.money.silver,
  copper: props.inventory.money.copper
})
function updateMoney(content: number, field: MoneyFields) {
  store().updateMoney(props.inventory.uuid, content, field)
}

watch(() => props.inventory.money, (newMoney) => {
  moneyFieldValues.value = {
    platinum: newMoney.platinum,
    gold: newMoney.gold,
    silver: newMoney.silver,
    copper: newMoney.copper
  }
})

const moneyOptions: [MoneyFields, number, string][] = [
  ['platinum', 1, 'PP'],
  ['gold', 2, 'GP'],
  ['silver', 3, 'SP'],
  ['copper', 4, 'CP']
]
</script>

<style scoped>
/* Chrome, Safari, Edge, Opera */
input::-webkit-outer-spin-button,
input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

/* Firefox */
input[type='number'] {
  appearance: textfield;
  -moz-appearance: textfield;
}
</style>
