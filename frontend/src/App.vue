<template>
  <div class="bg-background text-text w-screen">
    <div v-if="isLoggedIn" class="bg-background w-full">
      <div class="bg-primary-dark flex h-12 w-full items-center space-x-5 px-2 md:fixed md:top-0">
        <InvLogo class="h-10" />
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
        <div class="flex flex-1 items-center justify-end space-x-5 md:flex-initial">
          <button
            v-if="store().userIsDm"
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
          <button
            class="border-accent bg-primary-default h-10 w-10 rounded-sm border"
            @click="showSettings = true"
          >
            <FontAwesomeIcon :icon="faGears" />
          </button>
          <button
            class="border-accent bg-primary-default h-10 w-10 rounded-sm border"
            @click="logOut"
          >
            <FontAwesomeIcon :icon="faRightFromBracket" />
          </button>
        </div>
      </div>
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

      <div class="grid gap-5 overflow-auto p-5 md:mt-12 md:grid-cols-2 lg:grid-cols-3">
        <template v-if="Settings.getInstance().strictInventoryGrid">
          <InventoryContainer
            v-for="inventory in inventories"
            :key="inventory.uuid"
            :inventory="inventory"
          />
        </template>
        <template v-else>
          <div v-for="i in 3" :key="i" class="flex flex-col gap-5">
            <InventoryContainer
              v-for="inventory in getNthOf3Inventories(i - 1)"
              :key="inventory.uuid"
              :inventory="inventory"
            />
          </div>
        </template>
      </div>
    </div>
    <ErrorDisplay class="absolute bottom-0 z-50 w-screen" />
    <SettingsPopUp v-if="showSettings" @close="showSettings = false"></SettingsPopUp>
    <ManagePresetsPopUp
      v-if="showManagePresets"
      @close="showManagePresets = false"
    ></ManagePresetsPopUp>

    <button
      v-if="isLoggedIn"
      class="border-accent bg-primary-default fixed right-2 bottom-2 z-10 h-10 w-10 rounded-sm border"
      @click="showCreation = true"
    >
      <FontAwesomeIcon :icon="faPlus" />
    </button>
    <a
      :href="`https://github.com/NamelessGroup/InventarWerk/tree/${version}`"
      class="text-opacity-30 text-primary-text fixed bottom-1 left-1 hidden text-xs underline md:block"
      >{{ version }}</a
    >

    <div
      v-if="!acceptedCookies"
      class="bg-popup-background absolute top-0 right-0 bottom-0 left-0 z-40 p-10"
    >
      <div class="flex flex-col items-center space-y-2">
        <p>This website uses cookies.<br />No you can not reject.</p>
        <button
          class="border-accent bg-primary-default w-full rounded-sm border p-1 md:w-48"
          @click="acceptCookies"
        >
          Accept Cookies
        </button>
        <button
          class="border-accent bg-primary-default w-full rounded-sm border p-1 md:w-48"
          @click="acceptCookies"
        >
          Also Accept Cookies
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import InventoryContainer from './components/InventoryContainer.vue'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import {
  faGears,
  faList,
  faPlus,
  faRightFromBracket,
  faSearch,
  faUpload
} from '@fortawesome/free-solid-svg-icons'
import { store } from './store'
import ErrorDisplay from './errorHandling/ErrorDisplay.vue'
import { computed, ref } from 'vue'
import { DatabaseHandler } from './store/DatabaseHandler'
import PopUp from './components/PopUp.vue'
import { parseItems } from './utils/itemParser'
import SettingsPopUp from './components/SettingsPopUp.vue'
import ManagePresetsPopUp from './components/presetEditor/ManagePresetsPopUp.vue'
import { version } from './utils/version'
import type { Inventory } from './model/Inventory'
import { Settings } from './store/Settings'
import { ThemeManager } from './theme/ThemeManager.ts'
import InvLogo from './components/InvLogo.vue'

const showCreation = ref(false)
const nameFieldContent = ref('')
const errorContent = ref('')
const acceptedCookies = ref(false)
const showSettings = ref(false)
const showManagePresets = ref(false)

ThemeManager.getInstance().pageTheme = 'inventar'
getAcceptedCookies()

const searchString = ref('')
const search = computed(() => searchString.value.toLowerCase())

function matchSearchString(inventory: Inventory) {
  if (inventory.name.toLowerCase().includes(search.value)) {
    return true
  }
  return false
}

const inventories = computed(() => {
  const allInventories = store().inventoryUuids.map((uuid) => store().inventories[uuid])
  return allInventories.filter(matchSearchString)
})

function getNthOf3Inventories(n: number) {
  return inventories.value.filter((_, idx) => idx % 3 === n)
}

async function submitAddInventory() {
  if (nameFieldContent.value == '') {
    errorContent.value = 'Name cannot be empty'
    return
  }
  const result = await DatabaseHandler.getInstance().createInventory(nameFieldContent.value)
  if (result) {
    nameFieldContent.value = ''
    errorContent.value = ''
    showCreation.value = false
  } else {
    errorContent.value = 'Error creating inventory'
  }
}

const isLoggedIn = ref(false)

function checkLogIn() {
  DatabaseHandler.getInstance()
    .isLoggedIn()
    .then((res) => {
      isLoggedIn.value = res
      if (!res) {
        window.location.href = DatabaseHandler.getInstance().getLogInUrl()
      } else {
        DatabaseHandler.getInstance().initialize()
      }
    })
}

function acceptCookies() {
  const oldCookies = document.cookie
  acceptedCookies.value = true
  if (!oldCookies.includes('acceptedCookies=true')) {
    document.cookie = 'acceptedCookies=true'
  }
  checkLogIn()
}

function getAcceptedCookies() {
  const c1 = document.cookie.includes('acceptedCookies=true')
  if (c1) {
    acceptCookies()
    return
  }
  const c2 = window.location.search.includes('acceptCookies=true')
  if (c2) {
    acceptCookies()
  }
}

function logOut() {
  DatabaseHandler.getInstance()
    .logOut()
    .then(() => {
      window.location.reload()
    })
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
