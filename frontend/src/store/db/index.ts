import { AccountDB } from './accountDB'
import { LastChangesDB } from './changesDB'
import { InventoryDB } from './inventoryDB'
import { ZauberDB } from './zauberDB'

class DBFacade {
  private accountDB: AccountDB
  private changesDB: LastChangesDB
  private inventoryDB: InventoryDB
  private zauberDB: ZauberDB

  constructor() {
    this.accountDB = new AccountDB()
    this.inventoryDB = new InventoryDB()
    this.zauberDB = new ZauberDB()
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

  get zauber() {
    return this.zauberDB
  }

  public async initialize() {
    for (const db of [this.account, this.inventory, this.zauber, this.changes]) {
      await db.initialize()
    }
  }
}

export const DB = new DBFacade()
