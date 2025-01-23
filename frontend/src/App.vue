<template>
  <div class="w-screen text-white bg-slate-950">
    <div v-if="isLoggedIn" class="w-full bg-slate-950">
      <div class="flex h-12 w-full items-center justify-end space-x-5 bg-fuchsia-950 px-2 md:fixed md:top-0">
        <button v-if="store().userIsDm" class="h-10 w-10 rounded border border-amber-300 bg-fuchsia-900" @click="loadItemFile">
          <FontAwesomeIcon :icon="faUpload" />
        </button>
        <button class="h-10 w-10 rounded border border-amber-300 bg-fuchsia-900" @click="showSettings = true">
          <FontAwesomeIcon :icon="faGears" />
        </button>
        <button class="h-10 w-10 rounded border border-amber-300 bg-fuchsia-900" @click="logOut">
          <FontAwesomeIcon :icon="faRightFromBracket" />
        </button>
      </div>
      <PopUp v-if="showCreation" @close="showCreation = false">
        <div class="grid grid-cols-[auto_1fr] grid-rows-3 gap-2">
          <label for="name" class="row-start-1">Name:</label>
          <input
            id="name"
            v-model="nameFieldContent"
            type="text"
            class="row-start-1 rounded border border-amber-300 bg-fuchsia-900 px-1 outline-none"
          />
          <button
            class="col-span-2 row-start-2 rounded border border-amber-300 bg-fuchsia-900 p-1"
            @click="submitAddInventory"
          >
            Submit
          </button>
          <p class="col-span-2 row-start-3 text-red-500">{{ errorContent }}</p>
        </div>
      </PopUp>
      <div class="gap-5 overflow-auto p-5 grid md:grid-cols-2 lg:grid-cols-3 md:mt-12">
        <InventoryContainer
          v-for="uuid in store().inventoryUuids"
          :key="uuid"
          :inventory="store().inventories[uuid]"
        />
      </div>
    </div>
    <ErrorDisplay class="absolute bottom-0 z-50 w-screen" />
    <SettingsPopUp v-if="showSettings" @close="showSettings = false"></SettingsPopUp>

    <button
    v-if="isLoggedIn"
        class="fixed bottom-2 right-2 z-10 h-10 w-10 rounded border border-amber-300 bg-fuchsia-900"
        @click="showCreation = true"
      >
        <FontAwesomeIcon :icon="faPlus" />
      </button>

    <div
      v-if="!acceptedCookies"
      class="absolute bottom-0 left-0 right-0 top-0 z-40 bg-slate-900 p-10"
    >
      <div class="flex flex-col items-center space-y-2">
        <p>This website uses cookies.<br />No you can not reject.</p>
        <button
          class="w-full rounded border border-amber-300 bg-fuchsia-900 p-1 md:w-48"
          @click="acceptCookies"
        >
          Accept Cookies
        </button>
        <button
          class="w-full rounded border border-amber-300 bg-fuchsia-900 p-1 md:w-48"
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
import { faGears, faPlus, faRightFromBracket, faUpload } from '@fortawesome/free-solid-svg-icons'
import { store } from './store'
import ErrorDisplay from './errorHandling/ErrorDisplay.vue'
import { ref } from 'vue'
import { DatabaseHandler } from './store/DatabaseHandler'
import PopUp from './components/PopUp.vue'
import {parseItem} from './utils/itemParser'
import SettingsPopUp from './components/SettingsPopUp.vue'

const showCreation = ref(false)
const nameFieldContent = ref('')
const errorContent = ref('')
const acceptedCookies = ref(document.cookie.includes('acceptedCookies=true'))
const showSettings = ref(false)

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
if (acceptedCookies.value) {
  checkLogIn()
}

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
  if (oldCookies.includes('acceptedCookies=true')) {
    return
  }
  document.cookie = 'acceptedCookies=true'
  acceptedCookies.value = true
  checkLogIn()
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
    await parseItem(jsonContent)
  }
}


</script>
