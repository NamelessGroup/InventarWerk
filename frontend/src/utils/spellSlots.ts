import { SpellSlotState } from '@/model/SpellSlots'

export function generateSlotArray(max: number, available: number) {
  return Array.from({ length: max }, (_, i) =>
    i < available ? SpellSlotState.AVAILABLE : SpellSlotState.USED
  ).reverse()
}
