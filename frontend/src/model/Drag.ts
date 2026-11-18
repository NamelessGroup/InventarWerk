export type DraggableItem<T> = T & { sourceContainer: string }

export interface ItemGroup<I> {
  key: string
  label: string
  items: I[]
}
