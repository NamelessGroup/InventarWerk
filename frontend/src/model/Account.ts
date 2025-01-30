export interface RawAccount<T> {
  uuid: string
  name: string
  avatar: string | null
  dm: T
}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface Account extends RawAccount<boolean> {}

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
export interface DBAccount extends RawAccount<number> {}
