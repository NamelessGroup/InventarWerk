import type { SpellSlots } from '@/model/SpellSlots'
import { BaseDatabaseHandler, type QueryParameter } from './base'
import { zauberData } from '../pinia/zauber'
import { accountData } from '../pinia/account'
import { generateSlotArray } from '@/utils/spellSlots'
import type { DBSpellList } from '@/model/SpellList'
import type { SpellPreset } from '@/model/SpellPreset'

export class ZauberDB extends BaseDatabaseHandler {
  private static SPELL_SLOT_END_POINT = 'spellSlot'
  private static CONCENTRATION_END_POINT = 'concentration'
  private static SPELL_LIST_END_POINT = 'spellList'
  private static SPELL_PRESET_END_POINT = 'spellPreset'
  private static SPELL_END_POINT = this.SPELL_LIST_END_POINT + '/spell'

  public constructor() {
    super(BaseDatabaseHandler.getBaseURL(8001))
  }

  public async initialize(): Promise<void> {
    ;(await this.getAllPresets()).forEach((s) => zauberData().addPreset(s))
    zauberData().spellSlots[accountData().uuid] = (await this.getSpellSlots()) ?? {}
    zauberData().concentrationSpell[accountData().uuid] = await this.getConcentrationSpell()
    const spellLists = await this.getAllSpellLists()
    spellLists.forEach((s) => this.setSpellListInStore(s))
    zauberData().spellListUuids = spellLists.map((s) => s.uuid)
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
    const result = await this.get<{ spellPresetUuid: string }>(
      [ZauberDB.CONCENTRATION_END_POINT],
      uuid ? { owner_uuid: uuid } : {}
    )
    if (result) return result.spellPresetUuid
    return undefined
  }

  public async removeConcentration(uuid?: string) {
    const result = await this.delete<unknown>(
      [ZauberDB.CONCENTRATION_END_POINT],
      uuid ? { owner_uuid: uuid } : {}
    )
    return result !== undefined
  }

  public async createSpellList(name: string) {
    const result = await this.put<DBSpellList>([ZauberDB.SPELL_LIST_END_POINT], {
      spell_list_name: name
    })
    if (!result) return false

    this.setSpellListInStore(result)
    zauberData().spellListUuids.push(result.uuid)
    zauberData().spellListUuids.sort()
    return true
  }

  public async editSpellListName(uuid: string, name: string) {
    const result = await this.patch<unknown>([ZauberDB.SPELL_LIST_END_POINT, 'edit'], {
      spell_list_uuid: uuid,
      spell_list_name: name
    })
    return result !== undefined
  }

  public async deletePreset(uuid: string) {
    const result = await this.delete([ZauberDB.SPELL_PRESET_END_POINT, 'delete'], {
      spell_preset_uuid: uuid
    })
    return result !== undefined
  }

  public async deleteSpellList(uuid: string) {
    await this.delete<undefined>([ZauberDB.SPELL_LIST_END_POINT, 'delete'], {
      spell_list_uuid: uuid
    })
  }

  public async removeSpell(listUuid: string, spellUuid: string) {
    const result = await this.delete<unknown>([ZauberDB.SPELL_END_POINT, 'removePreset'], {
      spell_list_uuid: listUuid,
      spell_preset_uuid: spellUuid
    })
    return result !== undefined
  }

  public async getAllPresets() {
    const result = await this.get<{ spellPresets: SpellPreset[] }>([
      ZauberDB.SPELL_PRESET_END_POINT,
      'all'
    ])
    if (!result) return []
    return result.spellPresets
  }

  public async getAllSpellLists(): Promise<DBSpellList[]> {
    const result = await this.get<{ spell_lists: DBSpellList[] }>([
      ZauberDB.SPELL_LIST_END_POINT,
      'all'
    ])
    if (!result) return []
    return result.spell_lists
  }

  public async addSpellToList(listUuid: string, presetUuid: string) {
    const result = await this.put<unknown>([ZauberDB.SPELL_END_POINT, 'addPreset'], {
      spell_list_uuid: listUuid,
      spell_preset_uuid: presetUuid
    })
    return result !== undefined
  }

  private setSpellListInStore(spellList: DBSpellList) {
    zauberData().spellLists[spellList.uuid] = {
      ...spellList,
      entries: spellList.entries.map((e) => ({
        ...zauberData().getPreset(e.spellPresetUuid),
        sorting: e.sorting
      }))
    }
  }

  public getMassUploadURL() {
    return this.baseURL + ZauberDB.SPELL_PRESET_END_POINT + '/addExtern'
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
