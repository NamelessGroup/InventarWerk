<template>
  <div class="h-screen w-screen bg-slate-950 text-white">
    <div v-if="isLoggedIn" class="h-full w-full">
    <div class="flex h-12 w-full items-center justify-end space-x-5 bg-fuchsia-950 px-2">
      <button class="h-10 w-10 rounded border border-amber-300 bg-fuchsia-900">
        <FontAwesomeIcon :icon="faGears" />
      </button>
      <button class="h-10 w-10 rounded border border-amber-300 bg-fuchsia-900" @click="logOut">
        <FontAwesomeIcon :icon="faRightFromBracket" />
      </button>
    </div>
    <button class="absolute bottom-2 right-2 z-10 h-10 w-10 rounded border border-amber-300 bg-fuchsia-900" @click="showCreation = true">
      <FontAwesomeIcon :icon="faPlus" />
    </button>
    <PopUp v-if="showCreation" @close="showCreation = false">
      <div class="grid grid-cols-[auto_1fr] gap-2 grid-rows-3">
        <label for="name" class="row-start-1">Name:</label>
        <input id="name" v-model="nameFieldContent" type="text" class="row-start-1  rounded border border-amber-300 bg-fuchsia-900 outline-none px-1"/>  
        <button class="row-start-2 col-span-2 rounded border border-amber-300 bg-fuchsia-900 p-1" @click="submitAddInventory">Submit</button> 
        <p class="row-start-3 col-span-2 text-red-500">{{ errorContent }}</p>
      </div>
    </PopUp>
    <div class="p-5 space-y-5">
      <InventoryContainer
        v-for="uuid in store().inventoryUuids"
        :key="uuid"
        :inventory="store().inventories[uuid]"
      />
    </div>
  </div>
  <ErrorDisplay class="absolute z-50 bottom-0 w-screen" />

  <div v-if="!acceptedCookies" class="z-40 absolute top-0 left-0 right-0 bottom-0 bg-slate-900 p-10">
    <div class="flex flex-col items-center space-y-2">
      <p>This website uses cookies.<br />No you can not reject.</p>
      <button class="w-full md:w-48 rounded border border-amber-300 bg-fuchsia-900 p-1" @click="acceptCookies">Accept Cookies</button>
      <button class="w-full md:w-48 rounded border border-amber-300 bg-fuchsia-900 p-1" @click="acceptCookies">Also Accept Cookies</button>
    </div>
  </div>
  </div>
  
</template>

<script setup lang="ts">
import InventoryContainer from './components/InventoryContainer.vue'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faGears, faPlus, faRightFromBracket } from '@fortawesome/free-solid-svg-icons'
import { store } from './store';
import ErrorDisplay from './errorHandling/ErrorDisplay.vue';
import { ref } from 'vue';
import { DatabaseHandler } from './store/DatabaseHandler';
import PopUp from './components/PopUp.vue';

const showCreation = ref(false)
const nameFieldContent = ref('')
const errorContent = ref('')
const acceptedCookies = ref(document.cookie.includes('acceptedCookies=true'))

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
  DatabaseHandler.getInstance().isLoggedIn().then((res) => {
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
  DatabaseHandler.getInstance().logOut().then(() => {
    window.location.reload()
  })
}
</script>
