import { ErrorHandler } from "@/errorHandling/ErrorHandler"
import axios, { type AxiosResponse } from "axios"

export class DatabaseHandler {

  private static INSTANCE: DatabaseHandler|undefined
  private static BASE_URL = 'localhost:8080/'
  private lastFetch = 0

  private constructor() {
    this.fetchUpdates()
  }

  public getInstance() {
    if (DatabaseHandler.INSTANCE == undefined) {
      DatabaseHandler.INSTANCE = new DatabaseHandler()
    }
    return DatabaseHandler.INSTANCE
  }

  public async fetchUpdates() {
    // Time is stored here, so data that gets input between the request going out from the server and reaching the client is not lost
    const time = new Date().getTime()

    const inventoriesWithUpdates = await this.get<LastUpdateResponse>(['lastChanges'], {'timestamp': this.lastFetch.toString()})

    await Promise.all(inventoriesWithUpdates.map(update => {
      
    }))

    this.lastFetch = time
  }

  private async fetchInventory(uuid: string) {
    //await inventory = 
  }

  public createInventory(name: string) {

  }

  private async get<T>(url: URLParts, queryParams?: QueryParameter) {
    const params = new URLSearchParams(queryParams)
    const response = await axios.get<T>(DatabaseHandler.BASE_URL + url.join('/'), {params})
    if (this.wasSuccess(response)) {
      return response.data
    } else {
      ErrorHandler.getInstance().registerError(new Error(`Could not get ${url.join('/')}?${params.toString()} due to: ${response.status} ${response.statusText}`))
    }
  }

  private async post<T>(body: T, url: URLParts, queryParams?: QueryParameter) {
    const params = new URLSearchParams(queryParams)
    const response = await axios.post<T>(DatabaseHandler.BASE_URL + url.join('/'), {params, data: body})
    if (this.wasSuccess(response)) {
      return response.data
    } else {
      ErrorHandler.getInstance().registerError(new Error(`Could not get ${url.join('/')}?${params.toString()} due to: ${response.status} ${response.statusText}`))
    }
  }

  private wasSuccess(response: AxiosResponse) {
    return response.status >= 200 && response.status < 300
  }
}

type URLParts = string[]

type QueryParameter = Record<string, string>

type LastUpdateResponse = {uuid: string, type: 'create'|'patch'|'delete'}[]