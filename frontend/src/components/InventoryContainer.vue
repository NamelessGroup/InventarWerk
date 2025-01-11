<template>
  <div class="space-y-2 rounded border-2 border-amber-300 bg-fuchsia-950 p-2">
    <h1 class="bold text-xl">{{ inventory.name }}</h1>
    <div class="grid max-w-full grid-cols-4 gap-x-2">
      <input
        v-for="[k, i] of moneyOptions"
        :key="k"
        type="number"
        class="row-start-1 h-10 rounded border border-amber-300 bg-fuchsia-900 outline-none px-1"
        :value="inventory.money[k]"
        :class="`col-start-${i}`"
        @submit="e => evaluateMoneyString((e as unknown as any).target.value, k)"
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

    <button class="h-10 w-full rounded bg-fuchsia-900 text-center">+ Add item</button>
  </div>
</template>

<script setup lang="ts">
import { type PropType } from 'vue'
import type { Inventory } from '../model/Inventory'
import ItemRowDisplay from './ItemRowDisplay.vue'
import type { MoneyFields } from '@/utils/moneyMath';
import { store } from '@/store';
import { ErrorHandler } from '@/errorHandling/ErrorHandler';

const props = defineProps({
  inventory: {
    type: Object as PropType<Inventory>,
    required: true
  }
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
