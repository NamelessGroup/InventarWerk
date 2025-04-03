<template>
  <PopUp class="m-0!" @close="emit('close')">
    <div class="grid grid-cols-1 gap-2">
      <button
        class="w-full rounded-sm border border-amber-300 bg-fuchsia-900 p-1 outline-hidden"
        @click="addNewItem"
      >
        Add item
      </button>
      <label for="amount">Amount:</label>
      <NumericInput
        id="amount"
        v-model="amountValue"
        :default-value="1"
        class="w-full rounded-sm border border-amber-300 bg-fuchsia-900 px-1 outline-hidden"
      />
      <label for="name">Name:</label>
      <input
        id="name"
        ref="nameInput"
        v-model="nameValue"
        autocomplete="off"
        class="w-full rounded-sm border border-amber-300 bg-fuchsia-900 px-1 outline-hidden"
      />
      <div class="text-red-500">{{ errorText }}</div>
    </div>
  </PopUp>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import PopUp from './PopUp.vue'
import autocomplete, { type AutocompleteItem } from 'autocompleter'
import { store } from '@/store'
import { DatabaseHandler } from '@/store/DatabaseHandler'
import NumericInput from './NumericInput.vue'

const props = defineProps({
  inventoryUuid: {
    type: String,
    required: true
  }
})

const emit = defineEmits(['close'])

const nameInput = ref<HTMLInputElement | null>(null)
const nameValue = ref('')
const amountValue = ref(1)
const errorText = ref('')

const completionItems = computed(() => {
  const itemsInInventory = store().inventories[props.inventoryUuid].items.map(
    (item) => item.presetReference
  )
  return store()
    .itemPresets.map((item) => {
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
    .filter((i) => !itemsInInventory.includes(i.uuid))
    .sort((a, b) => {
      if (a.group == b.group) {
        return a.label.localeCompare(b.label)
      }
      if (a.group == 'Other') {
        return 1
      }
      if (b.group == 'Other') {
        return -1
      }
      return a.group.localeCompare(b.group)
    })
})

async function addNewItem() {
  if (nameValue.value === '') {
    errorText.value = 'Name cannot be empty'
    return
  }
  const success = await DatabaseHandler.getInstance().addNewItem(
    props.inventoryUuid,
    nameValue.value,
    amountValue.value
  )
  if (success) {
    errorText.value = ''
    nameValue.value = ''
    amountValue.value = 1
    emit('close')
  } else {
    errorText.value = 'Failed to add item'
  }
}

async function addItemByPreset(item: MyCompletionItem) {
  nameValue.value = item.label

  const success = await DatabaseHandler.getInstance().addItemByPreset(
    props.inventoryUuid,
    item.uuid,
    amountValue.value
  )
  if (success) {
    errorText.value = ''
    nameValue.value = ''
    amountValue.value = 1
    emit('close')
  } else {
    errorText.value = 'Failed to add item ' + item.label
  }
}

onMounted(() => {
  if (nameInput.value) {
    autocomplete<MyCompletionItem>({
      input: nameInput.value,
      fetch: (text, update) => {
        update(
          completionItems.value.filter((item) =>
            item.label.toLowerCase().includes(text.toLowerCase())
          )
        )
      },
      minLength: -1,
      showOnFocus: true,
      onSelect: (item) => {
        addItemByPreset(item)
      },
      className: 'autocomplete z-50 overflow-y-auto',
      render: (item: MyCompletionItem, currentValue: string): HTMLDivElement => {
        const div = document.createElement('div')
        const start = item.label.toLowerCase().indexOf(currentValue.toLowerCase())
        div.innerHTML = `${item.label.slice(0, start)}<span class="text-amber-300">${item.label.slice(start, start + currentValue.length)}</span>${item.label.slice(start + currentValue.length)}`
        div.classList.add(
          'bg-fuchsia-900',
          'px-1',
          'cursor-pointer',
          'pl-3',
          'text-white',
          'autocomplete-item'
        )
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
@reference "../style.css";

.selected.autocomplete-item {
  @apply bg-fuchsia-950;
}

.autocomplete-item:hover {
  @apply bg-fuchsia-700;
}

.autocomplete {
  @apply bottom-0 rounded-b;
}
</style>
