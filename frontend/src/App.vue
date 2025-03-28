<template>
  <div class="w-screen bg-slate-950 text-white">
    <div v-if="isLoggedIn" class="w-full bg-slate-950">
      <div class="flex h-12 w-full items-center space-x-5 bg-fuchsia-950 px-2 md:fixed md:top-0">
        <img src="./assets/logo.png" class="h-10" />
        <div class="flex flex-1 items-center justify-end space-x-5">
          <button
            v-if="store().userIsDm"
            class="h-10 w-10 rounded-sm border border-amber-300 bg-fuchsia-900"
            @click="loadItemFile"
          >
            <FontAwesomeIcon :icon="faUpload" />
          </button>
          <button
            class="h-10 w-10 rounded-sm border border-amber-300 bg-fuchsia-900"
            @click="showManagePresets = true"
          >
            <FontAwesomeIcon :icon="faList" />
          </button>
          <button
            class="h-10 w-10 rounded-sm border border-amber-300 bg-fuchsia-900"
            @click="showSettings = true"
          >
            <FontAwesomeIcon :icon="faGears" />
          </button>
          <button
            class="h-10 w-10 rounded-sm border border-amber-300 bg-fuchsia-900"
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
            class="row-start-1 rounded-sm border border-amber-300 bg-fuchsia-900 px-1 outline-hidden"
          />
          <button
            class="col-span-2 row-start-2 rounded-sm border border-amber-300 bg-fuchsia-900 p-1"
            @click="submitAddInventory"
          >
            Submit
          </button>
          <p class="col-span-2 row-start-3 text-red-500">{{ errorContent }}</p>
        </div>
      </PopUp>
      <div class="grid gap-5 overflow-auto p-5 md:mt-12 md:grid-cols-2 lg:grid-cols-3">
        <InventoryContainer
          v-for="uuid in store().inventoryUuids"
          :key="uuid"
          :inventory="store().inventories[uuid]"
        />
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
      class="fixed right-2 bottom-2 z-10 h-10 w-10 rounded-sm border border-amber-300 bg-fuchsia-900"
      @click="showCreation = true"
    >
      <FontAwesomeIcon :icon="faPlus" />
    </button>
    <a
      :href="`https://github.com/NamelessGroup/InventarWerk/tree/${version}`"
      class="text-opacity-30 fixed bottom-1 left-1 hidden text-xs text-fuchsia-300 underline md:block"
      >{{ version }}</a
    >

    <div
      v-if="!acceptedCookies"
      class="absolute top-0 right-0 bottom-0 left-0 z-40 bg-slate-900 p-10"
    >
      <div class="flex flex-col items-center space-y-2">
        <p>This website uses cookies.<br />No you can not reject.</p>
        <button
          class="w-full rounded-sm border border-amber-300 bg-fuchsia-900 p-1 md:w-48"
          @click="acceptCookies"
        >
          Accept Cookies
        </button>
        <button
          class="w-full rounded-sm border border-amber-300 bg-fuchsia-900 p-1 md:w-48"
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
  faUpload
} from '@fortawesome/free-solid-svg-icons'
import { store } from './store'
import ErrorDisplay from './errorHandling/ErrorDisplay.vue'
import { ref } from 'vue'
import { DatabaseHandler } from './store/DatabaseHandler'
import PopUp from './components/PopUp.vue'
import { parseItems } from './utils/itemParser'
import SettingsPopUp from './components/SettingsPopUp.vue'
import ManagePresetsPopUp from './components/presetEditor/ManagePresetsPopUp.vue'
import { version } from './utils/version'

const showCreation = ref(false)
const nameFieldContent = ref('')
const errorContent = ref('')
const acceptedCookies = ref(false)
const showSettings = ref(false)
const showManagePresets = ref(false)

getAcceptedCookies()

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
