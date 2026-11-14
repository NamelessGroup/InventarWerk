<template>
  <PopUp @close="close()">
    <div class="flex flex-col gap-5">
      <div class="flex items-center gap-2">
        <label for="name">Name:</label>
        <input
          id="name"
          ref="nameInput"
          v-model="nameValue"
          autocomplete="off"
          class="border-accent bg-primary-default w-full rounded-sm border px-1 outline-hidden"
        />
      </div>
      <div class="text-error-text">{{ errorText }}</div>
    </div>
  </PopUp>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import PopUp from '../PopUp.vue'
import { zauberData } from '@/store'
import autocomplete, { type AutocompleteItem } from 'autocompleter'

const props = defineProps({
  spellListUuid: {
    type: String,
    required: true
  }
})

const emit = defineEmits(['close'])

const nameInput = ref<HTMLInputElement | null>(null)
const nameValue = ref('')
const errorText = ref('')

function close() {
  nameValue.value = ''
  errorText.value = ''

  emit('close')
}

const completionItems = computed(() => {
  const spellsInLst = zauberData().spellLists[props.spellListUuid].entries.map((s) => s.uuid)
  return zauberData()
    .spellPresetList.map((s) => {
      let group = s.level === 0 ? 'Cantrip' : `Lvl ${s.level}`
      return {
        label: s.name,
        group: group,
        uuid: s.uuid
      }
    })
    .filter((i) => !spellsInLst.includes(i.uuid))
    .sort((a, b) => {
      if (a.group == b.group) {
        return a.label.localeCompare(b.label)
      }
      return a.group.localeCompare(b.group)
    })
})

function addSpell(mS: MyCompletionItem) {
  const spell = zauberData().getPreset(mS.uuid)
  if (spell === undefined) {
    errorText.value = 'Unknown Spell'
    return
  }
  zauberData().addSpellToList(props.spellListUuid, spell.uuid)
  close()
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
        addSpell(item)
      },
      className: 'autocomplete z-50 overflow-y-auto',
      render: (item: MyCompletionItem, currentValue: string): HTMLDivElement => {
        const div = document.createElement('div')
        const start = item.label.toLowerCase().indexOf(currentValue.toLowerCase())
        div.innerHTML = `${item.label.slice(0, start)}<span class="text-accent">${item.label.slice(start, start + currentValue.length)}</span>${item.label.slice(start + currentValue.length)}`
        div.classList.add(
          'bg-primary-default',
          'px-1',
          'cursor-pointer',
          'pl-3',
          'text-text',
          'autocomplete-item'
        )
        return div
      },
      renderGroup: (name: string): HTMLDivElement => {
        const div = document.createElement('div')
        div.textContent = name
        div.classList.add('bg-primary-default', 'p-1', 'text-text')
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
