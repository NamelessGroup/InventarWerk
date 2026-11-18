<template>
  <div>
    <PageBase class="h-full w-full" type="inventar">
      <template #header>
        <div class="flex-1"></div>
        <div
          class="border-accent bg-primary-default hidden max-w-96 flex-1 items-center justify-center gap-x-2 rounded border px-1 py-1 md:flex"
        >
          <FontAwesomeIcon :icon="faSearch" />
          <input
            v-model="searchString"
            class="flex-1 bg-transparent outline-none"
            placeholder="Search for inventories"
          />
        </div>
        <div class="flex-1"></div>
        <button
          v-if="accountData().userIsDm"
          class="border-accent bg-primary-default h-10 w-10 rounded-sm border"
          @click="loadItemFile"
        >
          <FontAwesomeIcon :icon="faUpload" />
        </button>
        <button
          class="border-accent bg-primary-default h-10 w-10 rounded-sm border"
          @click="showManagePresets = true"
        >
          <FontAwesomeIcon :icon="faList" />
        </button>
      </template>

      <template #default>
        <ContainerGrid class="overflow-auto" :containers="inventories">
          <template #default="{ item }">
            <InventoryContainer :inventory="item" />
          </template>
        </ContainerGrid>
      </template>
    </PageBase>

    <ManagePresetsPopUp
      v-if="showManagePresets"
      @close="showManagePresets = false"
    ></ManagePresetsPopUp>

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
          @click="submitAddInventory"
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
import { ThemeManager } from '@/theme/ThemeManager'
import { accountData, inventoryData } from '@/store'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faList, faPlus, faSearch, faUpload } from '@fortawesome/free-solid-svg-icons'
import InventoryContainer from '@/components/inventar/InventoryContainer.vue'
import PopUp from '@/components/PopUp.vue'
import ManagePresetsPopUp from '@/components/presetEditor/ManagePresetsPopUp.vue'
import { computed, ref } from 'vue'
import type { Inventory } from '@/model/Inventory'
import { parseItems } from '@/parser/itemParser'
import { DB } from '@/store/db'
import ContainerGrid from '@/components/ContainerGrid.vue'

ThemeManager.getInstance().pageTheme = 'inventar'

const showManagePresets = ref(false)
const showCreation = ref(false)
const nameFieldContent = ref('')
const errorContent = ref('')

const searchString = ref('')
const search = computed(() => searchString.value.toLowerCase())

function matchSearchString(inventory: Inventory) {
  if (inventory.name.toLowerCase().includes(search.value)) {
    return true
  }
  return false
}

const inventories = computed(() => {
  const allInventories = inventoryData().inventoryUuids.map(
    (uuid) => inventoryData().inventories[uuid]
  )
  return allInventories.filter(matchSearchString)
})

async function submitAddInventory() {
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

function loadItemFile() {
  let input = document.createElement('input')
  input.type = 'file'
  input.accept = '.zip,.json'
  input.multiple = false
  input.onchange = () => {
    const files = input.files
    if (!files) {
      return
    }
    const file = files.item(0)
    if (!file) {
      return
    }
    handleItemFile(file)
  }
  input.click()
}

function handleItemFile(file: File) {
  const fileReader = new FileReader()
  fileReader.onload = onItemFileRead
  fileReader.readAsText(file)

  async function onItemFileRead() {
    const textContent = fileReader.result as string
    const jsonContent = JSON.parse(textContent)
    await parseItems(jsonContent)
  }
}
</script>
