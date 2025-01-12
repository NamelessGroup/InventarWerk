import type { Inventory } from '@/model/Inventory'
import { breakDownMoney, compactMoney, type MoneyFields } from '@/utils/moneyMath'
import { defineStore } from 'pinia'

export const store = defineStore('store', {
  state: () => ({
    inventoryUuids: [],
    inventories: {},
    uuid: ''
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
    }
  }
})

interface State {
  inventoryUuids: string[]
  inventories: Record<string, Inventory>
  uuid: string
}
/*
enum ModificationSource {
  SERVER, USER
}*/