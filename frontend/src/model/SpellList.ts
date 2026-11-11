import type { SpellPreset } from './SpellPreset'

interface RawSpellList<T> {
  name: string
  ownerUuid: string
  spellUuids: T[]
  uuid: string
}

export type DBSpellList = RawSpellList<string>
export type SpellList = RawSpellList<SpellPreset>
