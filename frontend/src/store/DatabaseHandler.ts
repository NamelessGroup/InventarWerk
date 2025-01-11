import { ErrorHandler } from '@/errorHandling/ErrorHandler'
import type { Inventory, DBInventory } from '@/model/Inventory'
import { breakDownMoney, compactMoney } from '@/utils/moneyMath'
import axios, { type AxiosResponse } from 'axios'
import { store } from '.'
import type { ItemPreset } from '@/model/ItemPreset'

export class DatabaseHandler {
  private static INSTANCE: DatabaseHandler | undefined
  public static readonly BASE_URL = 'http://localhost:8000/'
  private static INVENTORY_END_POINT = 'inventory'
  private static ITEM_END_POINT = 'item'
  private static ITEM_PRESET_END_POINT = 'itemPreset'
  private static ACCOUNT_END_POINT = 'account'
  private lastFetch = 0

  private constructor() {
    //this.fetchUpdates()
  }

  public static getInstance() {
    if (DatabaseHandler.INSTANCE == undefined) {
      DatabaseHandler.INSTANCE = new DatabaseHandler()
    }
    return DatabaseHandler.INSTANCE
  }

  public async isLoggedIn() {
    const response = await this.get<{ logged_in: boolean }>([DatabaseHandler.ACCOUNT_END_POINT, 'isLoggedIn'])
    if (!response) {
      return false
    }
    return response.logged_in
  }

  public getLogInUrl() {
    return DatabaseHandler.BASE_URL + DatabaseHandler.ACCOUNT_END_POINT + '/login'
  }

  public async fetchUpdates() {
    // Time is stored here, so data that gets input between the request going out from the server and reaching the client is not lost
    const time = new Date().getTime()

    const inventoriesWithUpdates = await this.get<LastUpdateResponse>(['lastChanges'], {
      timestamp: this.lastFetch.toString()
    })

    if (!inventoriesWithUpdates) return

    /*await Promise.all(
      inventoriesWithUpdates.map(async (update) => {
        if (update.type == 'delete') {
          store().inventoryUuids = store().inventoryUuids.filter(u => u != update.uuid)
          delete store().inventories[update.uuid]
          return
        }
        
        const fetchResult = await this.fetchInventory(update.uuid)
        if (!fetchResult) {
          return
        }

        if (update.type == 'create') {
          store().inventoryUuids.push(update.uuid)
        }
      })
    )*/

    this.lastFetch = time
  }

  private async fetchInventory(uuid: string) {
    const inventory = await this.get<DBInventory>([DatabaseHandler.INVENTORY_END_POINT], { 'inventory_uuid': uuid })
    if (!inventory) return false

    this.setInventoryInStore(inventory)
    return true
  }

  public async createInventory(name: string) {
    const newInventory = await this.put<DBInventory>([DatabaseHandler.INVENTORY_END_POINT], { 'name': name })
    if (!newInventory) return false

    this.setInventoryInStore(newInventory)
    store().inventoryUuids.push(newInventory.uuid)
    return true
  }

  public async addItemByPreset(inventoryUuid: string, presetUuid: string, amount: number) {
    // Check if the inventory exists in the store
    const inventory = store().inventories[inventoryUuid];
    if (!inventory) return false

    // Fetch preset details from the server
    const presetData = await this.get<ItemPreset>([DatabaseHandler.ITEM_PRESET_END_POINT], { 'preset_uuid': presetUuid });
    if (!presetData) return false

    // Update the inventory in the backend
    await this.put<undefined>([DatabaseHandler.INVENTORY_END_POINT], { 'inventory_uuid': inventoryUuid, 'preset_uuid': presetUuid, 'amount': String(amount)});

    // Add the item to the inventory
    inventory.items.push({
      name: presetData.name,
      uuid: presetUuid,
      amount,
      dmNote: "",
      description: presetData.description,
      price: presetData.price,
      creator: presetData.creator,
      itemType: presetData.itemType
    });

    return true; 
  }

  public async getAllPresets() {
    const presets = await this.get<ItemPreset[]>([DatabaseHandler.ITEM_END_POINT, "all"]);
    if (!presets) return false
    return presets
  }

  private setInventoryInStore(inventory: DBInventory) {
    store().inventories[inventory.uuid] = {
      ...inventory,
      money: breakDownMoney(inventory.money)
    }
  }

  public patchMoney(inventory: Inventory) {
    const newMoney = compactMoney(inventory.money)

    this.patch([DatabaseHandler.INVENTORY_END_POINT, 'money'], { 'amount': newMoney.toString() })
  }

  private async get<T>(url: URLParts, queryParams?: QueryParameter) {
    const params = new URLSearchParams(queryParams)
    const response = await axios.get<T>(DatabaseHandler.BASE_URL + url.join('/'), { params, withCredentials:true }).then((response) => response).catch((error) => error.response)
    if (this.wasSuccess(response)) {
      return response.data
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
    const response = await axios.post<T>(DatabaseHandler.BASE_URL + url.join('/'), {
      params,
      withCredentials: true
    }).then((response) => response).catch((error) => error.response)
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
    const response = await axios.put<T>(DatabaseHandler.BASE_URL + url.join('/'), {
      params,
      withCredentials: true
    }).then((response) => response).catch((error) => error.response)
    if (this.wasSuccess(response)) {
      return response.data
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
    const response = await axios.patch<T>(DatabaseHandler.BASE_URL + url.join('/'), {
      params,
      withCredentials: true
    }).then((response) => response).catch((error) => error.response)
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

  private wasSuccess(response: AxiosResponse) {
    return response.status >= 200 && response.status < 300
  }
}

type URLParts = string[]

type QueryParameter = Record<string, string>

type LastUpdateResponse = { uuid: string; type: 'create' | 'patch' | 'delete' }[]