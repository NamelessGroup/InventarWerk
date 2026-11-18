export type SpellSlots = Record<number, SpellSlotState[]>

export enum SpellSlotState {
  AVAILABLE,
  USED
}
