<template>
  <div
    class="space-y-2 overflow-hidden rounded-sm border-2 border-amber-300 bg-fuchsia-950 p-2"
    @drop="dropItem"
    @dragover.prevent
    @dragenter.capture="(e) => dragEnterContainer(e)"
    @dragleave.self="(e) => dragLeaveContainer(e)"
  >
    <div class="flex items-center overflow-hidden">
      <DiscordImage :user="creator" class="h-6" />
      <div
        ref="nameInput"
        class="bold break-wrap ml-2 min-w-8 border-none bg-transparent pr-5 text-xl outline-hidden"
        :contenteditable="inventory.ownerUuid === store().uuid"
        @blur="updateName()"
        @keydown="
          (e) => {
            if (e.key === 'Enter') {
              e.preventDefault()
              e.stopPropagation()
              updateName()
            }
          }
        "
      >
        {{ inventory.name }}
      </div>
      <button
        v-if="inventory.ownerUuid === store().uuid"
        class="mr-2 h-7 w-7 shrink-0 rounded-sm border border-amber-300 bg-fuchsia-900"
        @click="editName()"
      >
        <FontAwesomeIcon :icon="faPen" />
      </button>
      <div class="mr-2 shrink-0">
        ({{ inventory.items.map((i) => i.amount * i.weight).reduce((a, b) => a + b, 0) }} lbs.)
      </div>
      <div class="flex-1"><!-- Spacer --></div>
      <button class="h-7 w-7 shrink-0 rounded-sm border border-amber-300 bg-fuchsia-900">
        <FontAwesomeIcon :icon="faShare" @click="showSharePopup = true" />
      </button>
      <button
        v-if="inventory.ownerUuid === store().uuid"
        class="ml-2 h-7 w-7 shrink-0 rounded-sm border border-amber-300 bg-fuchsia-900"
        @click="deleteInventory"
      >
        <FontAwesomeIcon :icon="faTrashCan" class="text-red-300" />
      </button>
    </div>
    <div class="grid max-w-full grid-cols-4 gap-x-2 overflow-auto">
      <NumericInput
        v-for="[k, i] of moneyOptions"
        :key="k"
        v-model="moneyFieldValues[k]"
        :readonly="!canEdit"
        class="row-start-1 h-10 rounded-sm border border-amber-300 bg-fuchsia-900 px-1 outline-hidden"
        :class="`col-start-${i}`"
        @update="(v) => updateMoney(v, k)"
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
      <template v-for="(item, idx) in inventory.items" :key="item.presetReference">
        <GhostItem
          v-if="ghostPosition === idx"
        />
        <ItemRowDisplay
          :can-edit="canEdit"
          :item="item"
          :inventory-uuid="inventory.uuid"
          @dragenter="(e) => dragEnterItem(idx, e)"
        />
      </template>

      <GhostItem v-if="ghostPosition === 0 && inventory.items.length === 0" />
    </div>

    <button
      v-if="inventory.writer.includes(store().uuid)"
      class="h-10 w-full rounded-sm bg-fuchsia-900 text-center"
      @click="showAddItemPopup = true"
    >
      + Add item
    </button>
  </div>

  <EditSharePopUp
    v-if="showSharePopup && store().uuid === inventory.ownerUuid"
    :inventory="inventory"
    @close="showSharePopup = false"
  />
  <ViewSharePopUp
    v-if="showSharePopup && store().uuid !== inventory.ownerUuid"
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
import type { Inventory } from '../model/Inventory'
import ItemRowDisplay from './ItemRowDisplay.vue'
import type { MoneyFields } from '@/utils/moneyMath'
import { store } from '@/store'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faPen, faShare, faTrashCan } from '@fortawesome/free-solid-svg-icons'
import AddItemPopUp from './AddItemPopUp.vue'
import EditSharePopUp from './share/EditSharePopUp.vue'
import NumericInput from './NumericInput.vue'
import DiscordImage from './DiscordImage.vue'
import ViewSharePopUp from './share/ViewSharePopUp.vue'
import GhostItem from './GhostItem.vue'

const props = defineProps({
  inventory: {
    type: Object as PropType<Inventory>,
    required: true
  }
})

const nameInput = ref<HTMLDivElement | null>(null)
const showSharePopup = ref(false)
const showAddItemPopup = ref(false)
const ghostPosition = ref(-1)
const currentlyHovering = ref<null | HTMLElement>(null);
const canEdit = computed(() => props.inventory.writer.includes(store().uuid))
const creator = computed(
  () =>
    store().accounts.filter((account) => account.uuid === props.inventory.ownerUuid)[0] ?? {
      name: 'Unknown',
      avatar: null,
      dm: false,
      uuid: ''
    }
)

function editName() {
  if (nameInput.value) {
    nameInput.value.focus()
  }
}

function updateName() {
  const newName = nameInput.value?.innerText ?? ''
  if (newName.length == 0) {
    return
  }
  if (newName == props.inventory.name) {
    return
  }
  store().editInventoryName(props.inventory.uuid, newName)
}

function deleteInventory() {
  const result = confirm('This will delete the inventory forever')
  if (!result) {
    return
  }
  store().deleteInventory(props.inventory.uuid)
}

function dragEnterContainer(e: DragEvent) {
  if (e.dataTransfer == null || e.dataTransfer.getData('type') !== 'item') {
    return
  }

  if (ghostPosition.value < 0) {
    ghostPosition.value = 0;
  }
  currentlyHovering.value = e.target as HTMLElement;
}

function dragLeaveContainer(e: DragEvent) {
  if (e.dataTransfer == null || e.dataTransfer.getData('type') !== 'item') {
    return
  }

  if (currentlyHovering.value === e.target) {
    ghostPosition.value = -1;
    currentlyHovering.value = null;
  }
}

function dragEnterItem(index: number, e: DragEvent) {
  if (e.dataTransfer == null || e.dataTransfer.getData('type') !== 'item') {
    return
  }
  
  ghostPosition.value = index
  currentlyHovering.value = e.target as HTMLElement
}

async function dropItem(e: DragEvent) {
  if (e.dataTransfer == null || e.dataTransfer.getData('type') !== 'item') {
    return
  }

  const sourceInventory = e.dataTransfer.getData('sourceInventory')
  const preset = e.dataTransfer.getData('preset')

  if (sourceInventory !== props.inventory.uuid && props.inventory.items.some((item) => item.presetReference === preset)) {
    ghostPosition.value = -1
    currentlyHovering.value = null
    throw new Error(`This inventory already contains the target item!`)
  }

  // Figuring out the new sorting value of every item in the inventory
  const sortedItems = [...props.inventory.items]
    .filter((item) => item.presetReference !== preset)
    .map((item) => ({ item: item.presetReference, sorting: -1, oldSorting: item.sorting }))
  sortedItems.splice(ghostPosition.value, 0, { item: preset, sorting: -1, oldSorting: -1 })
  sortedItems.forEach((item, idx) => item.sorting = idx)

  ghostPosition.value = -1
  currentlyHovering.value = null

  if (sourceInventory !== props.inventory.uuid) {
    store().moveItem(sourceInventory, props.inventory.uuid, preset)
  }

  for (const item of sortedItems) {
    if (item.oldSorting !== item.sorting) {
      store().changeItemSorting(props.inventory.uuid, item.item, item.sorting)
    }
  }
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
</style>
