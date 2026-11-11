import { defineStore } from 'pinia'
import { ref } from 'vue'
import { DB } from '../db'
import type { SpellList } from '@/model/SpellList'
import type { SpellPreset } from '@/model/SpellPreset'
import type { SpellSlots } from '@/model/SpellSlots'
import { accountData } from './account'
import { generateSlotArray } from '@/utils/spellSlots'

export const zauberData = defineStore('zauber', () => {
  const spellListUuids = ref<string[]>([])
  const spellLists = ref<Record<string, SpellList>>({})
  const spellPresets = ref<SpellPreset[]>([])
  const spellSlots = ref<Record<string, SpellSlots>>({})

  function setAvailableSlots(level: number, amount: number, uuid?: string) {
    if (!uuid) uuid = accountData().uuid
    spellSlots.value[uuid][level] = generateSlotArray(spellSlots.value[uuid][level].length, amount)
    DB.zauber.setAvailableSlots(level, amount, uuid)
  }

  function setSpellSlotMaxima(level: number, amount: number, uuid?: string) {
    if (!uuid) uuid = accountData().uuid
    const oldAmount = spellSlots.value[uuid][level].length
    let newAvailable = spellSlots.value[uuid][level].filter((v) => v).length
    if (amount > oldAmount) newAvailable += amount - oldAmount
    else if (amount > newAvailable) newAvailable = amount
    spellSlots.value[uuid][level] = generateSlotArray(amount, newAvailable)
    DB.zauber.setSpellSlotMaxima(level, amount, uuid)
  }

  function resetSpellSlots(uuid?: string) {
    if (!uuid) uuid = accountData().uuid
    //spellSlots.value[uuid] = generateSlotArray()
    // TODO
    DB.zauber.resetSpellSlots(uuid)
  }

  return {
    spellListUuids,
    spellLists,
    spellPresets,
    spellSlots,
    setAvailableSlots,
    setSpellSlotMaxima,
    resetSpellSlots
  }
})
