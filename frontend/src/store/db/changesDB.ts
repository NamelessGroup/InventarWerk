import { BaseDatabaseHandler } from './base'

export class LastChangesDB extends BaseDatabaseHandler {
  private changesEvent: EventSource | undefined

  public constructor() {
    super('')
  }

  public async initialize() {
    this.changesEvent = new EventSource(BaseDatabaseHandler.getBaseURL(8002) + 'lastChanges/')
    window.onbeforeunload = () => this.changesEvent?.close()
    //this.changesEvent.onmessage
  }
  /*
  public async fetchUpdates() {
      if (!inventoryData().isLoggedIn) return
      // Time is stored here, so data that gets input between the request going out from the server and reaching the client is not lost
      const time = new Date().getTime()

      const inventoriesWithUpdates = await this.get<LastUpdateResponse>(['lastChanges'])

      if (!inventoriesWithUpdates) return

      const keys = Object.keys(inventoriesWithUpdates)

      const deletedInventories = inventoryData().inventoryUuids.filter((uuid) => !keys.includes(uuid))
      for (const uuid of deletedInventories) {
        delete inventoryData().inventories[uuid]
        inventoryData().inventoryUuids = inventoryData().inventoryUuids.filter((u) => u != uuid)
      }

      for (const uuid of keys) {
        if (this.lastFetch <= inventoriesWithUpdates[uuid]) {
          await DB.inventory.fetchInventory(uuid)
        }
      }

      inventoryData().inventoryUuids = keys.sort()

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
  }*/
}
