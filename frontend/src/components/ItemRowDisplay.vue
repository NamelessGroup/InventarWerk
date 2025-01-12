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
      <input
        v-model="amountValue"
        type="text"
        autocomplete="off"
        class="row-start-1 h-8 w-10 rounded border-none bg-fuchsia-950 px-1 text-right outline-none"
        @click="
          (e) => {
            e.stopPropagation()
          }
        "
        @keydown="e => { if (e.key === 'Enter') { e.preventDefault(); editAmount() } }"
        @blur="e => editAmount()"
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
      <p class="text-xs">Price: {{ item.price }}</p>
      <p class="text-xs">{{ item.description }}</p>
      <p class="text-xs text-fuchsia-300">{{ item.dmNote }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Item } from '@/model/Item'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { ref, type PropType, watch } from 'vue'
import { faTrashCan } from '@fortawesome/free-solid-svg-icons'
import { store } from '@/store'
import { ErrorHandler } from '@/errorHandling/ErrorHandler'

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
const amountValue = ref(props.item.amount.toString())

function deleteItem() {
  store().removeItem(props.inventoryUuid, props.item.presetReference)
}

function editAmount() {
  let value = props.item.amount
  const content = amountValue.value
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

  store().changeItemAmount(props.inventoryUuid, props.item.presetReference, value)
}

watch(() => props.item.amount, (newValue) => {
  amountValue.value = newValue.toString()
})  
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
