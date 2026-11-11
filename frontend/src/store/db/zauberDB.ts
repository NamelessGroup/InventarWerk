import type { SpellSlots } from '@/model/SpellSlots'
import { BaseDatabaseHandler, type QueryParameter } from './base'
import { zauberData } from '../pinia/zauber'
import { accountData } from '../pinia/account'
import { generateSlotArray } from '@/utils/spellSlots'

export class ZauberDB extends BaseDatabaseHandler {
  private static SPELL_SLOT_END_POINT = 'spellSlot'

  public constructor() {
    super(BaseDatabaseHandler.getBaseURL(8001))
  }

  public async initialize(): Promise<void> {
    zauberData().spellSlots[accountData().uuid] = (await this.getSpellSlots()) ?? {}
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
