import type { SpellSlots } from '@/model/SpellSlots'
import { BaseDatabaseHandler, type QueryParameter } from './base'
import { zauberData } from '../pinia/zauber'
import { accountData } from '../pinia/account'
import { generateSlotArray } from '@/utils/spellSlots'

export class ZauberDB extends BaseDatabaseHandler {
  private static SPELL_SLOT_END_POINT = 'spellSlot'
  private static CONCENTRATION_END_POINT = 'concentration'
  private static SPELL_LIST_END_POINT = 'spellList'

  public constructor() {
    super(BaseDatabaseHandler.getBaseURL(8001))
  }

  public async initialize(): Promise<void> {
    zauberData().spellSlots[accountData().uuid] = (await this.getSpellSlots()) ?? {}
    zauberData().concentrationSpell[accountData().uuid] = await this.getConcentrationSpell()
  }

  public async getSpellSlots(uuid?: string): Promise<SpellSlots | undefined> {
    const result = await this.get<SpellSlotResult>(
      [ZauberDB.SPELL_SLOT_END_POINT],
      uuid ? { owner_uuid: uuid } : undefined
    )
    if (!result) return undefined
    const slots: SpellSlots = {}
    for (const s of result.slots) {
      slots[s.spellSlotLevel] = generateSlotArray(s.spellSlotMaxima, s.spellSlotAvailable)
    }
    for (let i = 0; i <= 9; i++) {
      if (slots[i] == undefined) slots[i] = []
    }
    return slots
  }

  public async setAvailableSlots(level: number, amount: number, uuid?: string) {
    const data: QueryParameter = { level: level.toString(), amount: amount.toString() }
    if (uuid) {
      data['owner_uuid'] = uuid
    }
    const result = await this.patch<unknown>([ZauberDB.SPELL_SLOT_END_POINT, 'available'], data)
    return result !== undefined
  }

  public async setSpellSlotMaxima(level: number, amount: number, uuid?: string) {
    const data: QueryParameter = { level: level.toString(), amount: amount.toString() }
    if (uuid) {
      data['owner_uuid'] = uuid
    }
    const result = await this.patch<unknown>([ZauberDB.SPELL_SLOT_END_POINT, 'maxima'], data)
    return result !== undefined
  }

  public async resetSpellSlots(uuid?: string) {
    const result = await this.patch<unknown>(
      [ZauberDB.SPELL_SLOT_END_POINT, 'reset'],
      uuid ? { owner_uuid: uuid } : undefined
    )
    return result !== undefined
  }

  public async getConcentrationSpell(uuid?: string) {
    const result = await this.get<{spellPresetUuid: string}>([ZauberDB.CONCENTRATION_END_POINT], uuid ? {uuid} : {})
    if (result) return result.spellPresetUuid
    return undefined
  }

  public async removeConcentration(uuid?: string) {
    const result = await this.delete<unknown>([ZauberDB.CONCENTRATION_END_POINT], uuid ? {uuid} : {})
    return result !== undefined
  }

  public async createSpellList() {
    
  }
}

interface SpellSlotResult {
  owner_uuid: string
  slots: {
    ownerUuid: string
    spellSlotAvailable: number
    spellSlotLevel: number
    spellSlotMaxima: number
  }[]
}
