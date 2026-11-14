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
  const spellPresets = ref<Record<string, SpellPreset>>({})
  const spellPresetList = ref<SpellPreset[]>([])
  const spellSlots = ref<Record<string, SpellSlots>>({})
  const concentrationSpell = ref<Record<string, string | undefined>>({})

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
    for (let i = 1; i <= 9; i++) {
      if (spellSlots.value[uuid][i] === undefined) continue
      const len = spellSlots.value[uuid][i].length
      spellSlots.value[uuid][i] = generateSlotArray(len, len)
    }
    DB.zauber.resetSpellSlots(uuid)
  }

  function editSpellListName(uuid: string, name: string) {
    spellLists.value[uuid].name = name
    DB.zauber.editSpellListName(uuid, name)
  }

  function deleteSpellList(uuid: string) {
    spellListUuids.value = spellListUuids.value.filter((s) => s !== uuid)
    DB.zauber.deleteSpellList(uuid)
  }

  function removeSpell(listUuid: string, spellUuid: string) {
    spellLists.value[listUuid].entries = spellLists.value[listUuid].entries.filter(
      (spell) => spell.uuid !== spellUuid
    )
    DB.zauber.removeSpell(listUuid, spellUuid)
  }

  function addPreset(spellPreset: SpellPreset) {
    spellPresetList.value.push(spellPreset)
    spellPresets.value[spellPreset.uuid] = spellPreset
  }

  function getPreset(uuid: string) {
    return spellPresets.value[uuid]
  }

  function deletePreset(uuid: string) {
    delete spellPresets.value[uuid]
    spellPresetList.value = spellPresetList.value.filter((s) => s.uuid !== uuid)
    DB.zauber.deletePreset(uuid)
  }

  function addSpellToList(listUuid: string, presetUuid: string) {
    spellLists.value[listUuid].entries.push({
      ...getPreset(presetUuid),
      sorting: Math.max(...spellLists.value[listUuid].entries.map((s) => s.sorting), 0) + 1
    })
    DB.zauber.addSpellToList(listUuid, presetUuid)
  }

  return {
    spellListUuids,
    spellLists,
    spellPresetList,
    spellSlots,
    setAvailableSlots,
    setSpellSlotMaxima,
    resetSpellSlots,
    concentrationSpell,
    editSpellListName,
    deleteSpellList,
    removeSpell,
    addPreset,
    getPreset,
    deletePreset,
    addSpellToList
  }
})
