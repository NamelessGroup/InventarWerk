<template>
  <PopUp @close="emit('close')">
    <div class="grid grid-cols-1 gap-2">
      <label for="amount">Amount:</label>
      <input id="amount" v-model="amountValue" class="rounded border border-amber-300 bg-fuchsia-900 outline-none px-1 w-full">
      <label for="name">Name:</label>
      <input id="name" ref="nameInput" v-model="nameValue" autocomplete="off" class="rounded border border-amber-300 bg-fuchsia-900 outline-none px-1 w-full">
      <button class="rounded border border-amber-300 bg-fuchsia-900 outline-none p-1 w-full" @click="addNewItem">Add item</button>
      <div class="text-red-500">{{ errorText }}</div>
    </div>
  </PopUp>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import PopUp from './PopUp.vue';
import autocomplete, { type AutocompleteItem } from 'autocompleter';
import { store } from '@/store';
import { DatabaseHandler } from '@/store/DatabaseHandler';

const props = defineProps({
  inventoryUuid: {
    type: String,
    required: true
  }
})

const emit = defineEmits(['close'])

const nameInput = ref<HTMLInputElement | null>(null)
const nameValue = ref('')
const amountValue = ref('1')
const errorText = ref('')

const completionItems = computed(() => {
  return store().itemPresets.map(item => {
    let group = item.itemType ?? 'Other'
    if (group === '') {
      group = 'Other'
    }
    return {
      label: item.name,
      group: group,
      uuid: item.uuid
    }
  })
})

async function addNewItem() {
  if (nameValue.value === '') {
    errorText.value = 'Name cannot be empty'
    return
  }
  const success = await DatabaseHandler.getInstance().addNewItem(props.inventoryUuid, nameValue.value, getAmount())
  if (success) {
    errorText.value = ''
    nameValue.value = ''
    amountValue.value = '1'
    emit('close')
  } else {
    errorText.value = 'Failed to add item'
  }
}

async function addItemByPreset(item: MyCompletionItem) {
  nameValue.value = item.label

  const success = await DatabaseHandler.getInstance().addItemByPreset(props.inventoryUuid, item.uuid, getAmount())
  if (success) {
    errorText.value = ''
    nameValue.value = ''
    amountValue.value = '1'
    emit('close')
  } else {
    errorText.value = 'Failed to add item ' + item.label
  }
}

function getAmount() {
  const result = parseInt(amountValue.value)
  if (isNaN(result)) {
    return 1
  }
  return result
}

onMounted(() => {
  if (nameInput.value) {
    autocomplete<MyCompletionItem>({
      input: nameInput.value,
      fetch: (text, update) => {
        update(completionItems.value.filter(item => item.label.toLowerCase().includes(text.toLowerCase())))
      },
      minLength: -1,
      showOnFocus: true,
      onSelect: (item) => {
        addItemByPreset(item)
      },
      className: 'autocomplete, z-50',
      render: (item: MyCompletionItem, currentValue: string): HTMLDivElement => {
        const div = document.createElement('div')
        const start = item.label.toLowerCase().indexOf(currentValue.toLowerCase())
        div.innerHTML = `${item.label.slice(0, start)}<span class="text-amber-300">${item.label.slice(start, start + currentValue.length)}</span>${item.label.slice(start + currentValue.length)}`
        div.classList.add('bg-fuchsia-900', 'px-1', 'cursor-pointer', 'pl-3', 'text-white', 'autocomplete-item')
        return div
      },
      renderGroup: (name: string): HTMLDivElement => {
        const div = document.createElement('div')
        div.textContent = name
        div.classList.add('bg-fuchsia-900', 'p-1', 'text-white')
        return div
      }
    })
  }
})

interface MyCompletionItem extends AutocompleteItem {
  label: string
  group: string
  uuid: string
}
</script>

<style lang="postcss">
.selected.autocomplete-item {
  @apply bg-fuchsia-950;
}

.autocomplete-item:hover {
  @apply bg-fuchsia-700;
}
</style>