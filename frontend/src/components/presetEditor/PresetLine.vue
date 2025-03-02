<!-- eslint-disable vue/no-v-html -->
<template>
  <div class="bg-fuchsia-900 px-1">
    <div class="flex items-center">
      <h3 class="flex-1" @click="showDetails = !showDetails">{{ preset.name }}</h3>
      <FontAwesomeIcon
        v-if="preset.creator == store().uuid"
        :icon="faTrashCan"
        class="cursor-pointer text-red-300"
        @click="
          (e) => {
            e.stopPropagation()
            deletePreset()
          }
        "
      />
    </div>
    <div v-if="showDetails">
      <p class="text-xs">Price: {{ priceString }}</p>
      <p class="mb-1 text-xs">Weight: {{ preset.weight }} lbs.</p>
      <p class="markdown mb-1 text-xs" v-html="description"></p>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ItemPreset } from '@/model/ItemPreset'
import { store } from '@/store'
import { DatabaseHandler } from '@/store/DatabaseHandler'
import { breakDownMoney, type MoneyFields } from '@/utils/moneyMath'
import { faTrashCan } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { marked } from 'marked'
import { computed, ref, type PropType } from 'vue'

const props = defineProps({
  preset: {
    type: Object as PropType<ItemPreset>,
    required: true
  }
})

const showDetails = ref(false)
const description = computed(() => {
  return marked(props.preset.description)
})

const moneySynonym: Record<MoneyFields, string> = {
  platinum: 'pp',
  gold: 'gp',
  silver: 'sp',
  copper: 'cp'
}

const priceString = computed(() => {
  if (props.preset.price == 0) {
    return '0cp'
  }
  const money = breakDownMoney(props.preset.price)
  const result = [] as string[]
  for (const k of ['platinum', 'gold', 'silver', 'copper'] as MoneyFields[]) {
    if (money[k] != 0) {
      result.push(`${money[k]}${moneySynonym[k]}`)
    }
  }

  return result.join(' ')
})

async function deletePreset() {
  const result = await DatabaseHandler.getInstance().deletePreset(props.preset.uuid)
  if (!result) {
    return
  }
  store().itemPresets = store().itemPresets.filter((p) => p.uuid != props.preset.uuid)
}
</script>
