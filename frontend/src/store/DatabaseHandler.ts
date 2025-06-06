import { ErrorHandler } from '@/errorHandling/ErrorHandler'
import type { DBInventory } from '@/model/Inventory'
import { breakDownMoney, compactMoney, type Money } from '@/utils/moneyMath'
import axios, { type AxiosResponse } from 'axios'
import { store } from '.'
import type { ItemPreset } from '@/model/ItemPreset'
import type { DBAccount } from '@/model/Account'
import { Settings } from './Settings'

export class DatabaseHandler {
  private static INSTANCE: DatabaseHandler | undefined
  public static readonly BASE_URL =
    import.meta.env.MODE == 'prod' ? `${window.location.origin}/` : 'http://localhost:8000/'
  private static INVENTORY_END_POINT = 'inventory'
  private static ITEM_END_POINT = 'item'
  private static ITEM_PRESET_END_POINT = 'itemPreset'
  private static ACCOUNT_END_POINT = 'account'
  private lastFetch = 0
  private fetchProcess: number | undefined

  private constructor() {
    this.setFetchInterval(Settings.getInstance().timeBetweenFetches)
  }

  public static getInstance() {
    if (DatabaseHandler.INSTANCE == undefined) {
      DatabaseHandler.INSTANCE = new DatabaseHandler()
    }
    return DatabaseHandler.INSTANCE
  }

  public async isLoggedIn() {
    const response = await this.get<{ loggedIn: boolean }>([
      DatabaseHandler.ACCOUNT_END_POINT,
      'isLoggedIn'
    ])
    if (!response) {
      return false
    }
    return response.loggedIn
  }

  public getLogInUrl() {
    return DatabaseHandler.BASE_URL + DatabaseHandler.ACCOUNT_END_POINT + '/login'
  }

  public async logOut() {
    await this.get<undefined>([DatabaseHandler.ACCOUNT_END_POINT, 'logout'])
  }

  public async fetchUpdates() {
    // Time is stored here, so data that gets input between the request going out from the server and reaching the client is not lost
    const time = new Date().getTime()

    const inventoriesWithUpdates = await this.get<LastUpdateResponse>(['lastChanges'])

    if (!inventoriesWithUpdates) return

    const keys = Object.keys(inventoriesWithUpdates)

    const deletedInventories = store().inventoryUuids.filter((uuid) => !keys.includes(uuid))
    for (const uuid of deletedInventories) {
      delete store().inventories[uuid]
      store().inventoryUuids = store().inventoryUuids.filter((u) => u != uuid)
    }

    for (const uuid of keys) {
      if (this.lastFetch <= inventoriesWithUpdates[uuid]) {
        await this.fetchInventory(uuid)
      }
    }

    store().inventoryUuids = keys.sort()

    this.lastFetch = time
  }

  public setFetchInterval(interval: number) {
    if (this.fetchProcess !== undefined) {
      clearInterval(this.fetchProcess)
    }
    if (interval == undefined) return
    this.fetchProcess = setInterval(() => {
      this.fetchUpdates()
    }, interval * 1000)
  }

  public async initialize() {
    store().uuid = await this.getOwnUUID()
    store().itemPresets = await this.getAllPresets()
    store().accounts = await this.getAllAccounts()
    store().userIsDm = await this.isDM()
    store().isServerLocked = (await this.getServerLockStatus()) ?? false
    const inventories = await this.getAllInventoriesFromDB()
    inventories.forEach((inventory) => this.setInventoryInStore(inventory))
    store().inventoryUuids = inventories.map((inventory) => inventory.uuid).sort()
  }

  public async isDM(user?: string) {
    const uuid = user ?? store().uuid
    const result = await this.get<{ isDm: boolean }>([DatabaseHandler.ACCOUNT_END_POINT, 'isDm'], {
      account_uuid: uuid
    })
    return result?.isDm ?? false
  }

  public async getAllAccounts() {
    const accounts =
      (await this.get<{ accounts: DBAccount[] }>([DatabaseHandler.ACCOUNT_END_POINT, 'get']).then(
        (r) => r?.accounts
      )) ?? []

    return accounts.map((account) => ({
      ...account,
      dm: account.dm == 1
    }))
  }

  private async getAllInventoriesFromDB() {
    return (
      (await this.get<{ inventories: DBInventory[] }>([
        DatabaseHandler.INVENTORY_END_POINT,
        'all'
      ]).then((r) => r?.inventories)) ?? []
    )
  }

  private async getOwnUUID() {
    return (
      (await this.get<{ userUUID: string }>([DatabaseHandler.ACCOUNT_END_POINT, 'info']).then(
        (r) => r?.userUUID
      )) ?? ''
    )
  }

  private async fetchInventory(uuid: string) {
    const inventory = await this.get<DBInventory>([DatabaseHandler.INVENTORY_END_POINT], {
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
    const result = await this.patch<unknown>([DatabaseHandler.ITEM_PRESET_END_POINT, 'modify'], {
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
    const result = await this.delete<unknown>([DatabaseHandler.ITEM_PRESET_END_POINT, 'delete'], {
      item_preset_uuid: presetUuid
    })

    return result !== undefined
  }

  public async getPreset(itemUuid: string) {
    return await this.get<ItemPreset>([DatabaseHandler.ITEM_PRESET_END_POINT], {
      item_preset_uuid: itemUuid
    })
  }

  public async createInventory(name: string) {
    const newInventory = await this.put<DBInventory>([DatabaseHandler.INVENTORY_END_POINT], {
      name: name
    })
    if (!newInventory) return false

    this.setInventoryInStore(newInventory)
    store().inventoryUuids.push(newInventory.uuid)
    store().inventoryUuids.sort()
    return true
  }

  public async addItemByPreset(inventoryUuid: string, presetUuid: string, amount: number) {
    // Fetch preset details from the server
    const presetData = await this.get<ItemPreset>([DatabaseHandler.ITEM_PRESET_END_POINT], {
      item_preset_uuid: presetUuid
    })
    if (!presetData) return false

    // Update the inventory in the backend
    const r = await this.put<unknown>(
      [DatabaseHandler.INVENTORY_END_POINT, DatabaseHandler.ITEM_END_POINT, 'addPreset'],
      { inventory_uuid: inventoryUuid, preset_uuid: presetUuid, amount: String(amount) }
    )

    if (r === undefined) return false

    // Add the item to the inventory
    store().inventories[inventoryUuid].items.push({
      name: presetData.name,
      presetReference: presetUuid,
      amount,
      dmNote: '',
      weight: presetData.weight,
      description: presetData.description,
      price: presetData.price,
      presetCreator: presetData.creator,
      itemType: presetData.itemType,
      sorting: Math.max(...store().inventories[inventoryUuid].items.map((i) => i.sorting), 0) + 1,
      inventoryItemNote: ''
    })

    return true
  }

  public async changeItemAmount(inventoryUuid: string, itemUuid: string, newAmount: number) {
    await this.patch<unknown>(
      [DatabaseHandler.INVENTORY_END_POINT, DatabaseHandler.ITEM_END_POINT, 'edit'],
      { inventory_uuid: inventoryUuid, item_preset_uuid: itemUuid, amount: newAmount.toString() }
    )
  }

  public async addNewItem(inventoryUuid: string, name: string, amount: number) {
    const response = await this.put<ItemPreset>(
      [DatabaseHandler.INVENTORY_END_POINT, DatabaseHandler.ITEM_END_POINT, 'addNew'],
      { inventory_uuid: inventoryUuid, name: name, amount: amount.toString() }
    )
    if (!response) return false

    store().itemPresets.push(response)
    store().inventories[inventoryUuid].items.push({
      name: response.name,
      presetReference: response.uuid,
      description: response.description,
      price: response.price,
      weight: response.weight,
      itemType: response.itemType,
      dmNote: '',
      inventoryItemNote: '',
      sorting: Math.max(...store().inventories[inventoryUuid].items.map((i) => i.sorting), 0) + 1,
      presetCreator: response.creator,
      amount
    })
    return true
  }

  public async removeItem(inventoryUuid: string, itemUuid: string) {
    await this.delete<unknown>(
      [DatabaseHandler.INVENTORY_END_POINT, DatabaseHandler.ITEM_END_POINT, 'remove'],
      { inventory_uuid: inventoryUuid, item_preset_uuid: itemUuid }
    )
  }

  public async getAllPresets() {
    const presets = await this.get<{ item_presets: ItemPreset[] }>([
      DatabaseHandler.ITEM_PRESET_END_POINT,
      'all'
    ])
    if (!presets) return []
    return presets.item_presets
  }

  private setInventoryInStore(inventory: DBInventory) {
    store().inventories[inventory.uuid] = {
      ...inventory,
      items: inventory.items.map((item) => ({
        ...item
      })),
      money: breakDownMoney(inventory.money)
    }
  }

  public async editItemNote(inventoryUuid: string, itemUuid: string, note: string) {
    const result = await this.patch<unknown>(
      [DatabaseHandler.INVENTORY_END_POINT, DatabaseHandler.ITEM_END_POINT, 'edit'],
      { inventory_uuid: inventoryUuid, item_preset_uuid: itemUuid, inventory_item_note: note }
    )
    return result !== undefined
  }

  public async editInventoryName(inventoryUuid: string, name: string) {
    const result = await this.patch<unknown>([DatabaseHandler.INVENTORY_END_POINT, 'edit'], {
      inventory_uuid: inventoryUuid,
      name: name
    })
    return result !== undefined
  }

  public async editDmNote(inventoryUuid: string, itemUuid: string, note: string) {
    const result = await this.patch<unknown>(
      [DatabaseHandler.INVENTORY_END_POINT, DatabaseHandler.ITEM_END_POINT, 'addNote'],
      { inventory_uuid: inventoryUuid, item_preset_uuid: itemUuid, note: note }
    )
    return result !== undefined
  }

  public async patchMoney(inventoryUuid: string, money: Money) {
    const newMoney = compactMoney(money)

    await this.patch([DatabaseHandler.INVENTORY_END_POINT, 'edit'], {
      inventory_uuid: inventoryUuid,
      amount: newMoney.toString()
    })
  }

  public async addShare(inventoryUuid: string, share: Share) {
    const params = this.buildShareParams(share)
    params['inventory_uuid'] = inventoryUuid
    await this.patch<undefined>([DatabaseHandler.INVENTORY_END_POINT, 'addShare'], params)
  }

  public async removeShare(inventoryUuid: string, share: Share) {
    const params = this.buildShareParams(share)
    params['inventory_uuid'] = inventoryUuid
    await this.patch<undefined>([DatabaseHandler.INVENTORY_END_POINT, 'removeShare'], params)
  }

  public async deleteInventory(inventoryUuid: string) {
    await this.delete<undefined>([DatabaseHandler.INVENTORY_END_POINT, 'delete'], {
      inventory_uuid: inventoryUuid
    })
  }

  public async getServerLockStatus() {
    return (await this.get<{ isLocked: boolean }>([DatabaseHandler.ACCOUNT_END_POINT, 'isLocked']))
      ?.isLocked
  }

  public changeServerLockStatus() {
    return this.patch<unknown>([DatabaseHandler.ACCOUNT_END_POINT, 'toggleLock'])
  }

  private buildShareParams(share: Share) {
    const params: Record<string, string> = {}
    if (share.reader_uuid) {
      params['reader_uuid'] = share.reader_uuid
    }
    if (share.writer_uuid) {
      params['writer_uuid'] = share.writer_uuid
    }
    return params
  }

  private async get<T>(url: URLParts, queryParams?: QueryParameter): Promise<T | undefined> {
    const params = new URLSearchParams(queryParams)
    const response = await axios
      .get<T>(DatabaseHandler.BASE_URL + url.join('/'), { params, withCredentials: true })
      .then((response) => response)
      .catch((error) => error.response)
    if (this.wasSuccess(response)) {
      return response.data as T
    } else {
      ErrorHandler.getInstance().registerError(
        new Error(
          `Could not get ${url.join('/')}?${params.toString()} due to: ${response.status} ${response.statusText}`
        )
      )
    }
  }

  private async post<T>(url: URLParts, queryParams?: QueryParameter) {
    const params = new URLSearchParams(queryParams)
    const response = await axios
      .post<T>(
        DatabaseHandler.BASE_URL + url.join('/'),
        {},
        {
          params,
          withCredentials: true
        }
      )
      .then((response) => response)
      .catch((error) => error.response)
    if (this.wasSuccess(response)) {
      return response.data
    } else {
      ErrorHandler.getInstance().registerError(
        new Error(
          `Could not post ${url.join('/')}?${params.toString()} due to: ${response.status} ${response.statusText}`
        )
      )
    }
  }

  private async put<T>(url: URLParts, queryParams?: QueryParameter) {
    const params = new URLSearchParams(queryParams)
    const response = await axios
      .put<T>(
        DatabaseHandler.BASE_URL + url.join('/'),
        {},
        {
          params,
          withCredentials: true
        }
      )
      .then((response) => response)
      .catch((error) => error.response)
    if (this.wasSuccess(response)) {
      return response.data as T
    } else {
      ErrorHandler.getInstance().registerError(
        new Error(
          `Could not put ${url.join('/')}?${params.toString()} due to: ${response.status} ${response.statusText}`
        )
      )
    }
  }

  private async patch<T>(url: URLParts, queryParams?: QueryParameter) {
    const params = new URLSearchParams(queryParams)
    const response = await axios
      .patch<T>(
        DatabaseHandler.BASE_URL + url.join('/'),
        {},
        {
          params,
          withCredentials: true
        }
      )
      .then((response) => response)
      .catch((error) => error.response)
    if (this.wasSuccess(response)) {
      return response.data
    } else {
      ErrorHandler.getInstance().registerError(
        new Error(
          `Could not patch ${url.join('/')}?${params.toString()} due to: ${response.status} ${response.statusText}`
        )
      )
    }
  }

  private async delete<T>(url: URLParts, queryParams?: QueryParameter) {
    const params = new URLSearchParams(queryParams)
    const response = await axios
      .delete<T>(DatabaseHandler.BASE_URL + url.join('/'), {
        params,
        withCredentials: true
      })
      .then((response) => response)
      .catch((error) => error.response)
    if (this.wasSuccess(response)) {
      return response.data
    } else {
      ErrorHandler.getInstance().registerError(
        new Error(
          `Could not delete ${url.join('/')}?${params.toString()} due to: ${response.status} ${response.statusText}`
        )
      )
    }
  }

  private wasSuccess(response: AxiosResponse) {
    return response.status >= 200 && response.status < 300
  }
}

type URLParts = string[]

type QueryParameter = Record<string, string>

type LastUpdateResponse = Record<string, number>

interface Share {
  reader_uuid?: string
  writer_uuid?: string
}
