<template>
  <div>
    <PageBase class="h-full w-full" type="zauber">
      <template #header>
        <div class="flex-1"></div>
        <div
          class="border-accent bg-primary-default hidden max-w-96 flex-1 items-center justify-center gap-x-2 rounded border px-1 py-1 md:flex"
        >
          <FontAwesomeIcon :icon="faSearch" />
          <input
            v-model="searchString"
            class="flex-1 bg-transparent outline-none"
            placeholder="Search for spell lists"
          />
        </div>
        <div class="flex-1"></div>
      </template>

      <template #default>
        <div class="flex flex-col gap-5 overflow-auto md:overflow-clip">
          <SpellSlotList
            v-if="zauberData().spellSlots[accountData().uuid] !== undefined"
            :spell-slots="zauberData().spellSlots[accountData().uuid]"
            :user="accountData().uuid"
            :concentration-spell="zauberData().concentrationSpell[accountData().uuid]"
          />

          <div class="min-h-[3000px] md:flex-1 md:overflow-auto"></div>
        </div>
      </template>
    </PageBase>

    <PopUp v-if="showCreation" @close="showCreation = false">
      <div class="grid grid-cols-[auto_1fr] grid-rows-3 gap-2">
        <label for="name" class="row-start-1">Name:</label>
        <input
          id="name"
          v-model="nameFieldContent"
          type="text"
          class="border-accent bg-primary-default row-start-1 rounded-sm border px-1 outline-hidden"
        />
        <button
          class="border-accent bg-primary-default col-span-2 row-start-2 rounded-sm border p-1"
          @click="() => {}"
        >
          Submit
        </button>
        <p class="text-error-text col-span-2 row-start-3">{{ errorContent }}</p>
      </div>
    </PopUp>

    <button
      class="border-accent bg-primary-default fixed right-2 bottom-2 z-10 h-10 w-10 rounded-sm border"
      @click="showCreation = true"
    >
      <FontAwesomeIcon :icon="faPlus" />
    </button>
  </div>
</template>

<script setup lang="ts">
import PageBase from '@/components/PageBase.vue'
import SpellSlotList from '@/components/zauber/SpellSlotList.vue'
import type { SpellList } from '@/model/SpellList'
import { accountData, zauberData } from '@/store'
import { ThemeManager } from '@/theme/ThemeManager'
import { faPlus, faSearch } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { computed, ref } from 'vue'
import PopUp from '@/components/PopUp.vue'
import { DB } from '@/store/db'

ThemeManager.getInstance().pageTheme = 'zauber'

const showCreation = ref(false)
const nameFieldContent = ref('')
const errorContent = ref('')

const searchString = ref('')
const search = computed(() => searchString.value.toLowerCase())

// eslint-disable-next-line @typescript-eslint/no-unused-vars
function matchSearchString(inventory: SpellList) {
  if (inventory.name.toLowerCase().includes(search.value)) {
    return true
  }
  return false
}

async function submitAddSpellList() {
  if (nameFieldContent.value == '') {
    errorContent.value = 'Name cannot be empty'
    return
  }
  const result = await DB.inventory.createInventory(nameFieldContent.value)
  if (result) {
    nameFieldContent.value = ''
    errorContent.value = ''
    showCreation.value = false
  } else {
    errorContent.value = 'Error creating inventory'
  }
}
</script>
