import type { Account } from '@/model/Account'
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { DB } from '../db'

export const accountData = defineStore('account', () => {
  const accounts = ref<Account[]>([])
  const userIsDm = ref(false)
  const isLoggedIn = ref(false)
  const isServerLocked = ref(false)
  const uuid = ref('')

  async function toggleLock() {
    if ((await DB.account.changeServerLockStatus()) !== undefined) {
      isServerLocked.value = !isServerLocked.value
    }
  }

  return { accounts, userIsDm, isLoggedIn, toggleLock, isServerLocked, uuid }
})
