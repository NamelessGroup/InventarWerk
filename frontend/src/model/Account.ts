export interface RawAccount<T> {
  uuid: string
  name: string
  dm: T
}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface Account extends RawAccount<boolean> {}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface DBAccount extends RawAccount<number> {}
