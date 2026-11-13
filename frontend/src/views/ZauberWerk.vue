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
        <button
          v-if="accountData().userIsDm"
          class="border-accent bg-primary-default h-10 w-10 rounded-sm border"
          @click="showAddExternPopUp = true"
        >
          <FontAwesomeIcon :icon="faUpload" />
        </button>
      </template>

      <template #default>
        <div class="flex flex-col gap-5 overflow-auto">
          <SpellSlotList
            v-if="zauberData().spellSlots[accountData().uuid] !== undefined"
            :spell-slots="zauberData().spellSlots[accountData().uuid]"
            :user="accountData().uuid"
            :concentration-spell="zauberData().concentrationSpell[accountData().uuid]"
          />

          <ContainerGrid class="flex-1" :containers="spellLists">
            <template #default="{ item }">
              <SpellListDisplay :spell-list="item" />
            </template>
          </ContainerGrid>
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
    <UploadPresetListPopUp v-if="showAddExternPopUp" @close="showAddExternPopUp = false" />

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
import { faPlus, faSearch, faUpload } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { computed, ref } from 'vue'
import PopUp from '@/components/PopUp.vue'
import { DB } from '@/store/db'
import UploadPresetListPopUp from '@/components/zauber/UploadPresetListPopUp.vue'
import ContainerGrid from '@/components/ContainerGrid.vue'
import SpellListDisplay from '@/components/zauber/SpellListDisplay.vue'

ThemeManager.getInstance().pageTheme = 'zauber'

const showCreation = ref(false)
const showAddExternPopUp = ref(false)
const nameFieldContent = ref('')
const errorContent = ref('')

const searchString = ref('')
const search = computed(() => searchString.value.toLowerCase())

function matchSearchString(spellList: SpellList) {
  if (spellList.name.toLowerCase().includes(search.value)) {
    return true
  }
  return false
}

const spellLists = computed(() => {
  const allInventories = zauberData().spellListUuids.map((uuid) => zauberData().spellLists[uuid])
  return allInventories.filter(matchSearchString)
})

// eslint-disable-next-line @typescript-eslint/no-unused-vars
async function submitAddSpellList() {
  if (nameFieldContent.value == '') {
    errorContent.value = 'Name cannot be empty'
    return
  }
  const result = await DB.zauber.createSpellList(nameFieldContent.value)
  if (result) {
    nameFieldContent.value = ''
    errorContent.value = ''
    showCreation.value = false
  } else {
    errorContent.value = 'Error creating spell list'
  }
}
</script>
