import type { Inventory } from '@/model/Inventory'
import { breakDownMoney, compactMoney, type MoneyFields } from '@/utils/moneyMath'
import { defineStore } from 'pinia'
import { DatabaseHandler } from './DatabaseHandler'
import type { Account } from '@/model/Account'

export const store = defineStore('store', {
  state: () => ({
    inventoryUuids: [],
    inventories: {},
    uuid: '',
    accounts: []
  } as State),
  getters: {
    getInvetory: (state) => (uuid: string): Inventory => {
      return state.inventories[uuid]
    },
  },
  actions: {
    updateMoney(inventoryUuid: string, newValue: number, field: MoneyFields) {
      const oldMoney = this.inventories[inventoryUuid].money
      oldMoney[field] = newValue
      const newMoney = breakDownMoney(compactMoney(oldMoney))
      this.inventories[inventoryUuid].money = newMoney
    },
    setName(inventoryUuid: string, newName: string) {
      this.inventories[inventoryUuid].name = newName
    },
    addReadShare(inventoryUuid: string, newShare: string) {
      this.inventories[inventoryUuid].reader.push(newShare)
      DatabaseHandler.getInstance().addShare(inventoryUuid, {read: [newShare]})
    },
    removeReadShare(inventoryUuid: string, shareToRemove: string) {
      this.inventories[inventoryUuid].reader = this.inventories[inventoryUuid].reader.filter(share => share !== shareToRemove)
      DatabaseHandler.getInstance().removeShare(inventoryUuid, {read: [shareToRemove]})
    },
    addWriteShare(inventoryUuid: string, newShare: string) {
      this.inventories[inventoryUuid].writer.push(newShare)
      DatabaseHandler.getInstance().addShare(inventoryUuid, {write: [newShare]})
    },
    removeWriteShare(inventoryUuid: string, shareToRemove: string) {
      this.inventories[inventoryUuid].writer = this.inventories[inventoryUuid].writer.filter(share => share !== shareToRemove)
      DatabaseHandler.getInstance().removeShare(inventoryUuid, {write: [shareToRemove]})
    },
    makePublic(inventoryUuid: string, allAccounts?: string[]) {
      if (allAccounts) {
        this.inventories[inventoryUuid].reader = allAccounts
      }
      DatabaseHandler.getInstance().addShare(inventoryUuid, {})
    },
    deleteInventory(inventoryUuid: string) {
      this.inventoryUuids = this.inventoryUuids.filter(uuid => uuid !== inventoryUuid)
      DatabaseHandler.getInstance().deleteInventory(inventoryUuid)
    }
  }
})

interface State {
  inventoryUuids: string[]
  inventories: Record<string, Inventory>
  uuid: string
  accounts: Account[]
}


/*
enum ModificationSource {
  SERVER, USER
}*/