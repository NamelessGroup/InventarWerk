<template>
  <PopUp @close="emit('close')">
    <div class="space-y-5 p-2">
      <div class="grid grid-cols-1 gap-2 md:grid-cols-[auto_1fr]">
        <label for="edit-item-name">Name:</label>
        <input id="edit-item-name" v-model="nameRef" type="text" />

        <label for="edit-item-type">Item type:</label>
        <input id="edit-item-type" v-model="typeRef" type="text" />

        <label for="edit-item-price">Price:</label>
        <NumericInput id="edit-item-price" v-model="priceRef" />

        <label for="edit-item-weight">Weight:</label>
        <NumericInput id="edit-item-weight" v-model="weightRef" />

        <label for="edit-item-description">Description:</label>
        <textarea id="edit-item-description" v-model="descriptionRef" class="min-h-60"></textarea>
      </div>
      <button class="w-full rounded border border-amber-300 bg-fuchsia-900 p-1" @click="saveItem">
        Save
      </button>
      <div class="text-red-500">{{ errorText }}</div>
    </div>
  </PopUp>
</template>

<script setup lang="ts">
import { ref, type PropType } from 'vue'
import PopUp from './PopUp.vue'
import type { Item } from '@/model/Item'
import { store } from '@/store'
import NumericInput from './NumericInput.vue'

const props = defineProps({
  inventoryUuid: {
    type: String,
    required: true
  },
  item: {
    type: Object as PropType<Item>,
    required: true
  }
})

const nameRef = ref(props.item.name)
const typeRef = ref(props.item.itemType)
const priceRef = ref(props.item.price)
const weightRef = ref(props.item.weight)
const descriptionRef = ref(props.item.description)

const emit = defineEmits(['close'])

const errorText = ref('')

async function saveItem() {
  const result = await store().editItem(props.inventoryUuid, props.item.presetReference, {
    name: nameRef.value,
    itemType: typeRef.value,
    price: priceRef.value,
    weight: weightRef.value,
    description: descriptionRef.value
  })
  if (result) {
    errorText.value = ''
    emit('close')
  }
  errorText.value = 'Failed to save item'
}
</script>

<style scoped lang="postcss">
input,
textarea {
  @apply rounded border border-amber-300 bg-fuchsia-900 p-1 outline-none;
}
</style>
