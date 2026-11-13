<template>
  <BaseContainer
    :container="inventory"
    :items="inventory.items"
    drag-key="presetReference"
    @update-name="(n) => inventoryData().editInventoryName(inventory.uuid, n)"
    @delete="inventoryData().deleteInventory(inventory.uuid)"
    @move="moveItem"
  >
    <template #extraInfo>
      <div class="shrink-0">
        ({{ inventory.items.map((i) => i.amount * i.weight).reduce((a, b) => a + b, 0) }} lbs.)
      </div>
    </template>
    <template #extraButtons>
      <button
        class="border-accent bg-primary-default h-7 w-7 shrink-0 rounded-sm border"
        @click="showSharePopup = true"
      >
        <FontAwesomeIcon :icon="faShare" />
      </button>
    </template>

    <template #preItems>
      <div class="grid max-w-full grid-cols-4 gap-x-2 overflow-auto">
        <NumericInput
          v-for="[k, i] of moneyOptions"
          :key="k"
          v-model="moneyFieldValues[k]"
          :readonly="!canEdit"
          class="border-accent bg-primary-default row-start-1 h-10 rounded-sm border px-1 outline-hidden"
          :class="`col-start-${i}`"
          @update="(v) => updateMoney(v, k)"
        />
        <span
          v-for="[k, i, l] of moneyOptions"
          :key="k + 'l'"
          :class="`col-start-${i}`"
          class="text-accent row-start-2 text-center text-sm"
          >{{ l }}</span
        >
      </div>
    </template>

    <template #item="{ item }">
      <ItemRowDisplay :can-edit="canEdit" :item="item" :inventory-uuid="inventory.uuid" />
    </template>

    <template #postItems>
      <button
        v-if="inventory.writer.includes(accountData().uuid)"
        class="bg-primary-default h-10 w-full rounded-sm text-center"
        @click="showAddItemPopup = true"
      >
        + Add item
      </button>
    </template>
  </BaseContainer>

  <EditSharePopUp
    v-if="showSharePopup && accountData().uuid === inventory.ownerUuid"
    :inventory="inventory"
    @close="showSharePopup = false"
  />
  <ViewSharePopUp
    v-if="showSharePopup && accountData().uuid !== inventory.ownerUuid"
    :inventory="inventory"
    @close="showSharePopup = false"
  />

  <AddItemPopUp
    v-if="showAddItemPopup"
    :inventory-uuid="inventory.uuid"
    @close="showAddItemPopup = false"
  />
</template>

<script setup lang="ts">
import { computed, ref, watch, type PropType } from 'vue'
import type { Inventory } from '@/model/Inventory'
import type { Item } from '@/model/Item'
import ItemRowDisplay from './ItemRowDisplay.vue'
import type { MoneyFields } from '@/utils/moneyMath'
import { accountData, inventoryData } from '@/store'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faShare } from '@fortawesome/free-solid-svg-icons'
import AddItemPopUp from './AddItemPopUp.vue'
import EditSharePopUp from '../share/EditSharePopUp.vue'
import NumericInput from '../NumericInput.vue'
import ViewSharePopUp from '../share/ViewSharePopUp.vue'
import BaseContainer from '../BaseContainer.vue'
import type { DraggableItem } from '../../model/Drag'

const props = defineProps({
  inventory: {
    type: Object as PropType<Inventory>,
    required: true
  }
})
const showSharePopup = ref(false)
const showAddItemPopup = ref(false)
const canEdit = computed(() => props.inventory.writer.includes(accountData().uuid))

async function moveItem(item: DraggableItem<Item>, newIndex: number, movedHere: boolean) {
  if (
    movedHere &&
    props.inventory.items.some(
      (existingItem) => existingItem.presetReference === item.presetReference
    )
  ) {
    throw new Error(`This inventory already contains ${item.name}!`)
  }

  // Figuring out the new sorting value of every item in the inventory
  const sortedItems = props.inventory.items
    .filter((existingItem) => existingItem.presetReference !== item.presetReference)
    .map((item) => ({ item: item.presetReference, sorting: -1, oldSorting: item.sorting }))
  sortedItems.splice(newIndex, 0, { item: item.presetReference, sorting: -1, oldSorting: -1 })
  sortedItems.forEach((item, idx) => (item.sorting = idx))

  if (movedHere) {
    await inventoryData().moveItem(
      item.sourceContainer,
      props.inventory.uuid,
      item.presetReference,
      newIndex
    )
  }

  const sortingsToUpdate: Record<string, number> = {}
  for (const sortingItem of sortedItems) {
    if (
      sortingItem.oldSorting !== sortingItem.sorting &&
      (!movedHere || sortingItem.item !== item.presetReference)
    ) {
      sortingsToUpdate[sortingItem.item] = sortingItem.sorting
    }
  }
  if (Object.keys(sortingsToUpdate).length > 0) {
    await inventoryData().changeInventorySorting(props.inventory.uuid, sortingsToUpdate)
  }
}

const moneyFieldValues = ref({
  platinum: props.inventory.money.platinum,
  gold: props.inventory.money.gold,
  silver: props.inventory.money.silver,
  copper: props.inventory.money.copper
})
function updateMoney(content: number, field: MoneyFields) {
  inventoryData().updateMoney(props.inventory.uuid, content, field)
}

watch(
  () => props.inventory.money,
  (newMoney) => {
    moneyFieldValues.value = {
      platinum: newMoney.platinum,
      gold: newMoney.gold,
      silver: newMoney.silver,
      copper: newMoney.copper
    }
  }
)

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

.fade-enter-active,
.fade-leave-active {
  transition: all 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateX(30px);
}
</style>
