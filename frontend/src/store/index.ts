import type { Inventory } from '@/model/Inventory'
import { breakDownMoney, compactMoney, type MoneyFields } from '@/utils/moneyMath'
import { defineStore } from 'pinia'

export const store = defineStore('store', {
  state: () => ({
    inventoryUuids: ['123'],
    inventories: {
      '123': {
        name: 'My Inventory',
        money: {copper: 3, silver: 6, gold: 4, platinum: 2},
        items: [
          {
            name: '123',
            uuid: 'string',
            amount: 2,
            dmNote: 'string',
            description: 'Tolle Beschreibeung',
            price: 123,
            creator: '',
            itemType: 'weapon'
          }
        ],
        uuid: '',
        writer: [],
        reader: [],
        owner: ''
      }
    }
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
}
/*
enum ModificationSource {
  SERVER, USER
}*/