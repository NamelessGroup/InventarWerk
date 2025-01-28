<template>
  <div
    class="grid rounded border bg-fuchsia-900 p-1"
    :class="{
      'border-amber-300': expanded,
      'border-fuchsia-900': !expanded
    }"
    @click="expanded = !expanded"
  >
    <div class="grid grid-cols-[auto_1fr_auto]">
      <NumericInput
        v-model="amountValue"
        class="row-start-1 h-8 w-10 rounded border-none bg-fuchsia-950 px-1 text-right outline-none"
        @click="
          (e: Event) => {
            e.stopPropagation()
          }
        "
        @update="(v) => editAmount(v)"
      />
      <span class="row-start-1 flex items-center px-2">{{ item.name }}</span>
      <button
        class="row-start-1 h-8 w-10 rounded border-none bg-fuchsia-950"
        @click="
          (e) => {
            e.stopPropagation()
            deleteItem()
          }
        "
      >
        <FontAwesomeIcon :icon="faTrashCan" class="text-red-300" />
      </button>
    </div>
    <div v-show="expanded">
      <div class="relative flex min-h-12 flex-col">
        <p class="text-xs">Price: {{ priceString }}</p>
        <p class="mb-1 text-xs">Weight: {{ item.weight }} lbs.</p>
        <p class="markdown mb-1 text-xs" v-html="description"></p>
        <textarea
          v-model="itemNote"
          class="rounded border border-amber-300 bg-fuchsia-900 text-xs text-fuchsia-300 outline-none"
          placeholder="Notes"
          @click="(e) => e.stopPropagation()"
          @blur="store().editItemNote(inventoryUuid, item.presetReference, itemNote)"
        ></textarea>
        <textarea
          v-if="store().userIsDm"
          v-model="dmNote"
          class="mt-1 rounded border border-amber-300 bg-fuchsia-900 text-xs text-amber-300 outline-none"
          placeholder="DM Note"
          @click="(e) => e.stopPropagation()"
          @blur="store().editDmNote(inventoryUuid, item.presetReference, dmNote)"
        ></textarea>

        <button
          v-if="store().uuid == item.presetCreator"
          class="absolute right-0 top-2 h-6 w-6 rounded border border-amber-300 bg-fuchsia-950 text-xs"
          @click="(e) => openEdit(e)"
        >
          <FontAwesomeIcon :icon="faPen" />
        </button>
      </div>
    </div>
  </div>
  <EditItemPopUp
    v-if="showItemEdit"
    :item="item"
    :inventory-uuid="inventoryUuid"
    @close="showItemEdit = false"
  />
</template>

<script setup lang="ts">
import type { Item } from '@/model/Item'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { ref, type PropType, watch, computed } from 'vue'
import { faPen, faTrashCan } from '@fortawesome/free-solid-svg-icons'
import { store } from '@/store'
import { marked } from 'marked'
import EditItemPopUp from './EditItemPopUp.vue'
import { breakDownMoney, type MoneyFields } from '@/utils/moneyMath'
import NumericInput from './NumericInput.vue'

const props = defineProps({
  item: {
    type: Object as PropType<Item>,
    required: true
  },
  inventoryUuid: {
    type: String,
    required: true
  }
})

const expanded = ref(false)
const amountValue = ref(props.item.amount)
const description = computed(() => marked.parse(props.item.description))
const itemNote = ref(props.item.inventoryItemNote)
const dmNote = ref(props.item.dmNote)

const moneySynonym: Record<MoneyFields, string> = {
  platinum: 'pp',
  gold: 'gp',
  silver: 'sp',
  copper: 'cp'
}

const priceString = computed(() => {
  if (props.item.price == 0) {
    return '0cp'
  }
  const money = breakDownMoney(props.item.price)
  const result = [] as string[]
  for (const k of ['platinum', 'gold', 'silver', 'copper'] as MoneyFields[]) {
    if (money[k] != 0) {
      result.push(`${money[k]}${moneySynonym[k]}`)
    }
  }

  return result.join(' ')
})

function deleteItem() {
  store().removeItem(props.inventoryUuid, props.item.presetReference)
}

function editAmount(value: number) {
  store().changeItemAmount(props.inventoryUuid, props.item.presetReference, value)
}

const showItemEdit = ref(false)
function openEdit(e: Event) {
  e.stopPropagation()
  showItemEdit.value = true
}

watch(
  () => props.item.amount,
  (newValue) => {
    amountValue.value = newValue
  }
)
watch(
  () => props.item.inventoryItemNote,
  (newValue) => {
    itemNote.value = newValue
  }
)
watch(
  () => props.item.dmNote,
  (newValue) => {
    dmNote.value = newValue
  }
)
</script>

<style scoped lang="postcss">
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
