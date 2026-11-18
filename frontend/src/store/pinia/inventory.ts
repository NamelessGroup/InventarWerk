import type { Inventory } from '@/model/Inventory'
import type { ItemPreset } from '@/model/ItemPreset'
import { breakDownMoney, compactMoney, type MoneyFields } from '@/utils/moneyMath'
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { DB } from '../db'

export const inventoryData = defineStore('inventory', () => {
  const inventoryUuids = ref<string[]>([])
  const inventories = ref<Record<string, Inventory>>({})
  const itemPresets = ref<ItemPreset[]>([])

  function getInventory(uuid: string): Inventory {
    return inventories.value[uuid]
  }

  function updateMoney(inventoryUuid: string, newValue: number, field: MoneyFields) {
    const oldMoney = inventories.value[inventoryUuid].money
    oldMoney[field] = newValue
    const newMoney = breakDownMoney(compactMoney(oldMoney))
    inventories.value[inventoryUuid].money = newMoney

    DB.inventory.patchMoney(inventoryUuid, newMoney)
  }
  function setName(inventoryUuid: string, newName: string) {
    inventories.value[inventoryUuid].name = newName
  }
  async function addReadShare(inventoryUuid: string, newShare: string) {
    inventories.value[inventoryUuid].reader.push(newShare)
    await DB.inventory.setShare(inventoryUuid, newShare, 'r')
  }
  async function removeReadShare(inventoryUuid: string, shareToRemove: string) {
    inventories.value[inventoryUuid].reader = inventories.value[inventoryUuid].reader.filter(
      (share) => share !== shareToRemove
    )
    await DB.inventory.setShare(inventoryUuid, shareToRemove)
  }
  async function addWriteShare(inventoryUuid: string, newShare: string) {
    inventories.value[inventoryUuid].writer.push(newShare)
    await DB.inventory.setShare(inventoryUuid, newShare, 'w')
  }
  async function removeWriteShare(inventoryUuid: string, shareToRemove: string) {
    inventories.value[inventoryUuid].writer = inventories.value[inventoryUuid].writer.filter(
      (share) => share !== shareToRemove
    )
    await DB.inventory.setShare(inventoryUuid, shareToRemove, 'r')
  }
  function makePublic(inventoryUuid: string, allAccounts?: string[]) {
    if (allAccounts) {
      inventories.value[inventoryUuid].reader = allAccounts
    }
    return DB.inventory.setShare(inventoryUuid)
  }
  function deleteInventory(inventoryUuid: string) {
    inventoryUuids.value = inventoryUuids.value.filter((uuid) => uuid !== inventoryUuid)
    DB.inventory.deleteInventory(inventoryUuid)
  }
  function editInventoryName(inventoryUuid: string, newName: string) {
    inventories.value[inventoryUuid].name = newName
    DB.inventory.editInventoryName(inventoryUuid, newName)
  }
  function removeItem(inventoryUuid: string, itemUuid: string) {
    inventories.value[inventoryUuid].items = inventories.value[inventoryUuid].items.filter(
      (item) => item.presetReference !== itemUuid
    )
    DB.inventory.removeItem(inventoryUuid, itemUuid)
  }
  function changeItemAmount(inventoryUuid: string, itemUuid: string, newAmount: number) {
    inventories.value[inventoryUuid].items.find(
      (item) => item.presetReference === itemUuid
    )!.amount = newAmount
    DB.inventory.changeItemAmount(inventoryUuid, itemUuid, newAmount)
  }
  async function changeInventorySorting(
    inventoryUuid: string,
    newSortings: Record<string, number>
  ) {
    const items = inventories.value[inventoryUuid].items
    const promises: Promise<void>[] = []
    for (const itemToSort in newSortings) {
      const item = items.find((item) => item.presetReference === itemToSort)
      if (item != null) {
        item.sorting = newSortings[itemToSort]
        promises.push(
          DB.inventory.changeItemSorting(inventoryUuid, itemToSort, newSortings[itemToSort])
        )
      }
    }
    items.sort((a, b) => a.sorting - b.sorting)
    await Promise.all(promises)
  }

  async function editItem(
    inventoryUuid: string,
    itemUuid: string,
    changes: {
      name: string
      description: string
      price: number
      weight: number
      itemType: string
    }
  ) {
    const result = await DB.inventory.editItem(itemUuid, changes)
    if (!result) {
      return false
    }
    const item = inventories.value[inventoryUuid].items.find(
      (item) => item.presetReference === itemUuid
    )!
    item.name = changes.name
    item.description = changes.description
    item.price = changes.price
    item.weight = changes.weight
    item.itemType = changes.itemType
    return true
  }
  async function editItemNote(inventoryUuid: string, itemUuid: string, note: string) {
    inventories.value[inventoryUuid].items.find(
      (item) => item.presetReference === itemUuid
    )!.inventoryItemNote = note
    await DB.inventory.editItemNote(inventoryUuid, itemUuid, note)
  }
  async function editDmNote(inventoryUuid: string, itemUuid: string, note: string) {
    inventories.value[inventoryUuid].items.find(
      (item) => item.presetReference === itemUuid
    )!.dmNote = note
    await DB.inventory.editDmNote(inventoryUuid, itemUuid, note)
  }
  async function moveItem(
    sourceInventoryUuid: string,
    targetInventoryUuid: string,
    itemUuid: string,
    newSorting: number
  ) {
    const item = inventories.value[sourceInventoryUuid].items.find(
      (item) => item.presetReference === itemUuid
    )!
    inventories.value[sourceInventoryUuid].items = inventories.value[
      sourceInventoryUuid
    ].items.filter((item) => item.presetReference !== itemUuid)
    if (newSorting != null) {
      item.sorting = newSorting
      inventories.value[targetInventoryUuid].items.splice(newSorting, 0, item)
    } else {
      inventories.value[targetInventoryUuid].items.push(item)
    }
    await DB.inventory.moveItem(sourceInventoryUuid, targetInventoryUuid, itemUuid, newSorting)
  }

  return {
    itemPresets,
    getInventory,
    updateMoney,
    setName,
    addReadShare,
    removeReadShare,
    addWriteShare,
    removeWriteShare,
    makePublic,
    editDmNote,
    editInventoryName,
    editItem,
    editItemNote,
    deleteInventory,
    moveItem,
    changeInventorySorting,
    changeItemAmount,
    removeItem,
    inventories,
    inventoryUuids
  }
})
