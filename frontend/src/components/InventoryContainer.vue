<template>
  <div class="space-y-2 rounded border-2 border-amber-300 bg-fuchsia-950 p-2">
    <div class="flex gap-2">
      <input ref="nameInput" v-model="inventoryName" :readonly="true" class="bold text-xl bg-transparent outline-none border-none max-w-96 flex-1" @blur="updateName">
      <button v-if="false" class="rounded border border-amber-300 bg-fuchsia-900 w-7 h-7"  @click="editName">
        <FontAwesomeIcon :icon="faPen" />
      </button>
      <div class="flex-1"><!-- Spacer --></div>
      <button v-if="inventory.owner === store().uuid" class="rounded border border-amber-300 bg-fuchsia-900 w-7 h-7">
        <FontAwesomeIcon :icon="faShare" />
      </button>
      <button v-if="inventory.owner === store().uuid" class="rounded border border-amber-300 bg-fuchsia-900 w-7 h-7" @click="deleteInventory">
        <FontAwesomeIcon :icon="faTrashCan" class="text-red-300" />
      </button>
    </div>
    <div class="grid max-w-full grid-cols-4 gap-x-2">
      <input
        v-for="[k, i] of moneyOptions"
        :key="k"
        v-model="moneyFieldValues[k]"
        class="row-start-1 h-10 rounded border border-amber-300 bg-fuchsia-900 outline-none px-1"
        :class="`col-start-${i}`"
        @keydown="e => { if (e.key === 'Enter') { evaluateMoneyString((e as unknown as any).target.value, k) } }"
        @blur="e => evaluateMoneyString(moneyFieldValues[k], k)"
      />
      <span
        v-for="[k, i, l] of moneyOptions"
        :key="k + 'l'"
        :class="`col-start-${i}`"
        class="row-start-2 text-center text-sm text-amber-200"
        >{{ l }}</span
      >
    </div>

    <div>
      <ItemRowDisplay v-for="item in inventory.items" :key="item.uuid" :item="item" />
    </div>

    <button class="h-10 w-full rounded bg-fuchsia-900 text-center" @click="showAddItemPopup = true">+ Add item</button>
  </div>
  <PopUp v-if="showSharePopup">
  </PopUp>
  <AddItemPopUp v-if="showAddItemPopup" :inventory-uuid="inventory.uuid" @close="showAddItemPopup = false" />
</template>

<script setup lang="ts">
import { ref, watch, type PropType } from 'vue'
import type { Inventory } from '../model/Inventory'
import ItemRowDisplay from './ItemRowDisplay.vue'
import type { MoneyFields } from '@/utils/moneyMath';
import { store } from '@/store';
import { ErrorHandler } from '@/errorHandling/ErrorHandler';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';
import { faPen, faShare, faTrashCan } from '@fortawesome/free-solid-svg-icons';
import PopUp from './PopUp.vue';
import AddItemPopUp from './AddItemPopUp.vue';

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

function editName() {
  if (nameInput.value) {
    nameInput.value.focus()
  }
}

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
  platinum: props.inventory.money.platinum.toString(),
  gold: props.inventory.money.gold.toString(),
  silver: props.inventory.money.silver.toString(),
  copper: props.inventory.money.copper.toString()
})
function evaluateMoneyString(content: string, field: MoneyFields) {
  let value = props.inventory.money[field]
  if (content == '') {
    value = 0
  } else if (content.match(/^[+-]?\d+$/)) {
    value = parseInt(content)
  } else {
    try {
      value = eval(content)
    } catch (e) {
      ErrorHandler.getInstance().registerError(e as Error)
    }
  }

  store().updateMoney(props.inventory.uuid, value, field)
}

watch(() => props.inventory.money, (newMoney) => {
  moneyFieldValues.value = {
    platinum: newMoney.platinum.toString(),
    gold: newMoney.gold.toString(),
    silver: newMoney.silver.toString(),
    copper: newMoney.copper.toString()
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
