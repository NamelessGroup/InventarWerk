import { AccountDB } from './accountDB'
import { LastChangesDB } from './changesDB'
import { InventoryDB } from './inventoryDB'

class DBFacade {
  private accountDB: AccountDB
  private changesDB: LastChangesDB
  private inventoryDB: InventoryDB

  constructor() {
    this.accountDB = new AccountDB()
    this.inventoryDB = new InventoryDB()
    this.changesDB = new LastChangesDB()
  }

  get account() {
    return this.accountDB
  }

  get changes() {
    return this.changesDB
  }

  get inventory() {
    return this.inventoryDB
  }

  public async initialize() {
    for (const db of [this.account, this.changes, this.inventory]) {
      await db.initialize()
    }
  }
}

export const DB = new DBFacade()
