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
      DatabaseHandler.getInstance().addShare(inventoryUuid, {reader_uuid: newShare})
    },
    removeReadShare(inventoryUuid: string, shareToRemove: string) {
      this.inventories[inventoryUuid].reader = this.inventories[inventoryUuid].reader.filter(share => share !== shareToRemove)
      DatabaseHandler.getInstance().removeShare(inventoryUuid, {reader_uuid: shareToRemove})
    },
    addWriteShare(inventoryUuid: string, newShare: string) {
      this.inventories[inventoryUuid].writer.push(newShare)
      DatabaseHandler.getInstance().addShare(inventoryUuid, {writer_uuid: newShare})
    },
    removeWriteShare(inventoryUuid: string, shareToRemove: string) {
      this.inventories[inventoryUuid].writer = this.inventories[inventoryUuid].writer.filter(share => share !== shareToRemove)
      DatabaseHandler.getInstance().removeShare(inventoryUuid, {writer_uuid: shareToRemove})
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
    },
    async editItem(inventoryUuid: string, itemUuid: string, changes: {
      name: string, description: string, price: number, weight: number, itemType: string }) {
        const result = await DatabaseHandler.getInstance().editItem(itemUuid, changes)
        if (!result) {
          return false
        }
        const item = this.inventories[inventoryUuid].items.find(item => item.presetReference === itemUuid)!
        item.name = changes.name
        item.description = changes.description
        item.price = changes.price
        item.weight = changes.weight
        item.itemType = changes.itemType
        return true
    },
    async editItemNote(inventoryUuid: string, itemUuid: string, note: string) {
      this.inventories[inventoryUuid].items.find(item => item.presetReference === itemUuid)!.inventoryItemNote = note
      await DatabaseHandler.getInstance().editItemNote(inventoryUuid, itemUuid, note)
    },
    async editDmNote(inventoryUuid: string, itemUuid: string, note: string) {
      this.inventories[inventoryUuid].items.find(item => item.presetReference === itemUuid)!.dmNote = note
      await DatabaseHandler.getInstance().editDmNote(inventoryUuid, itemUuid, note)
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