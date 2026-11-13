import type { SpellPreset } from './SpellPreset'

interface RawSpellList {
  name: string
  ownerUuid: string
  uuid: string
}

export interface DBSpellList extends RawSpellList {
  spellUuids: string[]
}
export interface SpellList extends RawSpellList {
  spells: SpellPreset[]
}
