import { AccountDB } from './accountDB'
import { CastStream } from './castStream'
import { LastChangesStream } from './changesDB'
import { InventoryDB } from './inventoryDB'
import { ZauberDB } from './zauberDB'

class DBFacade {
  private accountDB: AccountDB
  private changesDB: LastChangesStream
  private inventoryDB: InventoryDB
  private zauberDB: ZauberDB
  private castStream: CastStream

  constructor() {
    this.accountDB = new AccountDB()
    this.inventoryDB = new InventoryDB()
    this.zauberDB = new ZauberDB()
    this.changesDB = new LastChangesStream()
    this.castStream = new CastStream()
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

  get cast() {
    return this.castStream
  }

  public async initialize() {
    for (const db of [this.account, this.inventory, this.zauber, this.changes, this.castStream]) {
      await db.initialize()
    }
  }
}

export const DB = new DBFacade()
