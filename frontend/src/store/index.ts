import type { Inventory } from '@/model/Inventory'
import { breakDownMoney, compactMoney, type MoneyFields } from '@/utils/moneyMath'
import { defineStore } from 'pinia'
import { DatabaseHandler } from './DatabaseHandler'
import type { Account } from '@/model/Account'
import type { ItemPreset } from '@/model/ItemPreset'

export const store = defineStore('store', {
  state: () => ({
    inventoryUuids: [],
    inventories: {},
    uuid: '',
    accounts: [],
    itemPresets: [],
    userIsDm: false
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

      DatabaseHandler.getInstance().patchMoney(inventoryUuid, newMoney)
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
      return DatabaseHandler.getInstance().addShare(inventoryUuid, {})
    },
    deleteInventory(inventoryUuid: string) {
      this.inventoryUuids = this.inventoryUuids.filter(uuid => uuid !== inventoryUuid)
      DatabaseHandler.getInstance().deleteInventory(inventoryUuid)
    },
    removeItem(inventoryUuid: string, itemUuid: string) {
      this.inventories[inventoryUuid].items = this.inventories[inventoryUuid].items.filter(item => item.presetReference !== itemUuid)
      DatabaseHandler.getInstance().removeItem(inventoryUuid, itemUuid)
    },
    changeItemAmount(inventoryUuid: string, itemUuid: string, newAmount: number) {
      this.inventories[inventoryUuid].items.find(item => item.presetReference === itemUuid)!.amount = newAmount
      DatabaseHandler.getInstance().changeItemAmount(inventoryUuid, itemUuid, newAmount)
    }
  }
})

interface State {
  inventoryUuids: string[]
  inventories: Record<string, Inventory>
  uuid: string
  accounts: Account[]
  itemPresets: ItemPreset[]
  userIsDm: boolean
}


/*
enum ModificationSource {
  SERVER, USER
}*/