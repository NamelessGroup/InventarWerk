import type { DBInventory } from '@/model/Inventory'
import { inventoryData } from '..'
import { BaseDatabaseHandler } from './base'
import { breakDownMoney, compactMoney, type Money } from '@/utils/moneyMath'
import type { ItemPreset } from '@/model/ItemPreset'

export class InventoryDB extends BaseDatabaseHandler {
  private static INVENTORY_END_POINT = 'inventory'
  private static ITEM_END_POINT = 'item'
  private static ITEM_PRESET_END_POINT = 'itemPreset'

  public constructor() {
    super(BaseDatabaseHandler.getBaseURL(8000))
  }

  public async initialize(): Promise<void> {
    inventoryData().itemPresets = await this.getAllPresets()

    const inventories = await this.getAllInventoriesFromDB()
    inventories.forEach((inventory) => this.setInventoryInStore(inventory))
    inventoryData().inventoryUuids = inventories.map((inventory) => inventory.uuid).sort()
  }

  private async getAllInventoriesFromDB() {
    return (
      (await this.get<{ inventories: DBInventory[] }>([
        InventoryDB.INVENTORY_END_POINT,
        'all'
      ]).then((r) => r?.inventories)) ?? []
    )
  }

  public async fetchInventory(uuid: string) {
    const inventory = await this.get<DBInventory>([InventoryDB.INVENTORY_END_POINT], {
      inventory_uuid: uuid
    })
    if (!inventory) return false

    this.setInventoryInStore(inventory)
    return true
  }

  public async editItem(
    itemUuid: string,
    settings: {
      name: string
      price: number
      weight: number
      description: string
      itemType: string
    }
  ) {
    const result = await this.patch<unknown>([InventoryDB.ITEM_PRESET_END_POINT, 'modify'], {
      item_preset_uuid: itemUuid,
      item_type: settings.itemType,
      name: settings.name,
      price: settings.price.toString(),
      weight: settings.weight.toString(),
      description: settings.description
    })

    return result !== undefined
  }

  public async deletePreset(presetUuid: string) {
    const result = await this.delete<unknown>([InventoryDB.ITEM_PRESET_END_POINT, 'delete'], {
      item_preset_uuid: presetUuid
    })

    return result !== undefined
  }

  public async getPreset(itemUuid: string) {
    return await this.get<ItemPreset>([InventoryDB.ITEM_PRESET_END_POINT], {
      item_preset_uuid: itemUuid
    })
  }

  public async createInventory(name: string) {
    const newInventory = await this.put<DBInventory>([InventoryDB.INVENTORY_END_POINT], {
      name: name
    })
    if (!newInventory) return false

    this.setInventoryInStore(newInventory)
    inventoryData().inventoryUuids.push(newInventory.uuid)
    inventoryData().inventoryUuids.sort()
    return true
  }

  public async addItemByPreset(inventoryUuid: string, presetUuid: string, amount: number) {
    // Fetch preset details from the server
    const presetData = await this.get<ItemPreset>([InventoryDB.ITEM_PRESET_END_POINT], {
      item_preset_uuid: presetUuid
    })
    if (!presetData) return false

    // Update the inventory in the backend
    const r = await this.put<unknown>(
      [InventoryDB.INVENTORY_END_POINT, InventoryDB.ITEM_END_POINT, 'addPreset'],
      { inventory_uuid: inventoryUuid, preset_uuid: presetUuid, amount: String(amount) }
    )

    if (r === undefined) return false

    // Add the item to the inventory
    inventoryData().inventories[inventoryUuid].items.push({
      name: presetData.name,
      presetReference: presetUuid,
      amount,
      dmNote: '',
      weight: presetData.weight,
      description: presetData.description,
      price: presetData.price,
      presetCreator: presetData.creator,
      itemType: presetData.itemType,
      sorting:
        Math.max(...inventoryData().inventories[inventoryUuid].items.map((i) => i.sorting), 0) + 1,
      inventoryItemNote: ''
    })

    return true
  }

  public async changeItemAmount(inventoryUuid: string, itemUuid: string, newAmount: number) {
    await this.patch<unknown>(
      [InventoryDB.INVENTORY_END_POINT, InventoryDB.ITEM_END_POINT, 'edit'],
      { inventory_uuid: inventoryUuid, item_preset_uuid: itemUuid, amount: newAmount.toString() }
    )
  }

  public async changeItemSorting(inventoryUuid: string, itemUuid: string, newSorting: number) {
    await this.patch<unknown>(
      [InventoryDB.INVENTORY_END_POINT, InventoryDB.ITEM_END_POINT, 'edit'],
      { inventory_uuid: inventoryUuid, item_preset_uuid: itemUuid, sorting: newSorting.toString() }
    )
  }

  public async addNewItem(inventoryUuid: string, name: string, amount: number) {
    const response = await this.put<ItemPreset>(
      [InventoryDB.INVENTORY_END_POINT, InventoryDB.ITEM_END_POINT, 'addNew'],
      { inventory_uuid: inventoryUuid, name: name, amount: amount.toString() }
    )
    if (!response) return false

    inventoryData().itemPresets.push(response)
    inventoryData().inventories[inventoryUuid].items.push({
      name: response.name,
      presetReference: response.uuid,
      description: response.description,
      price: response.price,
      weight: response.weight,
      itemType: response.itemType,
      dmNote: '',
      inventoryItemNote: '',
      sorting:
        Math.max(...inventoryData().inventories[inventoryUuid].items.map((i) => i.sorting), 0) + 1,
      presetCreator: response.creator,
      amount
    })
    return true
  }

  public async removeItem(inventoryUuid: string, itemUuid: string) {
    await this.delete<unknown>(
      [InventoryDB.INVENTORY_END_POINT, InventoryDB.ITEM_END_POINT, 'remove'],
      { inventory_uuid: inventoryUuid, item_preset_uuid: itemUuid }
    )
  }

  public async moveItem(
    sourceInventoryUuid: string,
    targetInventoryUuid: string,
    itemUuid: string,
    newSorting: number
  ) {
    await this.patch<unknown>(
      [InventoryDB.INVENTORY_END_POINT, InventoryDB.ITEM_END_POINT, 'move'],
      {
        source_inventory_uuid: sourceInventoryUuid,
        target_inventory_uuid: targetInventoryUuid,
        item_preset_uuid: itemUuid,
        new_sorting: newSorting.toString()
      }
    )
  }

  public async getAllPresets() {
    const presets = await this.get<{ item_presets: ItemPreset[] }>([
      InventoryDB.ITEM_PRESET_END_POINT,
      'all'
    ])
    if (!presets) return []
    return presets.item_presets
  }

  private setInventoryInStore(inventory: DBInventory) {
    inventoryData().inventories[inventory.uuid] = {
      ...inventory,
      items: inventory.items.map((item) => ({
        ...item
      })),
      money: breakDownMoney(inventory.money)
    }
  }

  public async editItemNote(inventoryUuid: string, itemUuid: string, note: string) {
    const result = await this.patch<unknown>(
      [InventoryDB.INVENTORY_END_POINT, InventoryDB.ITEM_END_POINT, 'edit'],
      { inventory_uuid: inventoryUuid, item_preset_uuid: itemUuid, inventory_item_note: note }
    )
    return result !== undefined
  }

  public async editInventoryName(inventoryUuid: string, name: string) {
    const result = await this.patch<unknown>([InventoryDB.INVENTORY_END_POINT, 'edit'], {
      inventory_uuid: inventoryUuid,
      name: name
    })
    return result !== undefined
  }

  public async editDmNote(inventoryUuid: string, itemUuid: string, note: string) {
    const result = await this.patch<unknown>(
      [InventoryDB.INVENTORY_END_POINT, InventoryDB.ITEM_END_POINT, 'addNote'],
      { inventory_uuid: inventoryUuid, item_preset_uuid: itemUuid, note: note }
    )
    return result !== undefined
  }

  public async patchMoney(inventoryUuid: string, money: Money) {
    const newMoney = compactMoney(money)

    await this.patch([InventoryDB.INVENTORY_END_POINT, 'edit'], {
      inventory_uuid: inventoryUuid,
      amount: newMoney.toString()
    })
  }

  public async setShare(inventoryUuid: string, memberUuid?: string, shareType?: 'r' | 'w') {
    const params: Record<string, string> = {}
    params['inventory_uuid'] = inventoryUuid

    if (memberUuid) {
      params['member_uuid'] = memberUuid
    }

    if (shareType) {
      params['share_type'] = shareType
    }

    await this.patch<undefined>([InventoryDB.INVENTORY_END_POINT, 'share'], params)
  }

  public async deleteInventory(inventoryUuid: string) {
    await this.delete<undefined>([InventoryDB.INVENTORY_END_POINT, 'delete'], {
      inventory_uuid: inventoryUuid
    })
  }

  public getMassUploadURL() {
    return this.baseURL + InventoryDB.ITEM_PRESET_END_POINT + '/addExtern'
  }
}
