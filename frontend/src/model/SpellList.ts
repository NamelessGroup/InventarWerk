import type { SpellPreset } from './SpellPreset'

interface RawSpellList<E> {
  name: string
  ownerUuid: string
  uuid: string
  entries: E[]
}

export type DBSpellList = RawSpellList<{
  sorting: number
  spellPresetUuid: string
}>
export interface SpellEntry extends SpellPreset {
  sorting: number
}
export type SpellList = RawSpellList<SpellEntry>
