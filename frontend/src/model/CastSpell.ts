import type { Account } from './Account'
import type { SpellPreset } from './SpellPreset'

export interface CastSpell {
  caster: Account
  spell: SpellPreset
  castOptions: {
    level: number
    ritual: boolean
    endedConcentrationOn: SpellPreset | undefined
  }
}
