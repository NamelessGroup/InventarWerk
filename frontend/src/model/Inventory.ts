import type { Money } from "@/utils/moneyMath"
import type { Item } from "./Item"

interface RawInventory<T> {
  uuid: string
  name: string
  money: T
  owner: string
  reader: string[]
  writer: string[]
  items: Item[]
}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface Inventory extends RawInventory<Money> {
}


// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface DBInventory extends RawInventory<number> {
}