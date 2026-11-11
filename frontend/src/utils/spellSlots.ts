export function generateSlotArray(max: number, available: number) {
  return Array.from({ length: max }, (_, i) => i < available).reverse()
}
