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

  function endConcentration(uuid?: string) {
    if (!uuid) uuid = accountData().uuid
    concentrationSpell.value[uuid] = undefined
    DB.zauber.removeConcentration(uuid)
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

  async function changeSpellListSorting(listUuid: string, newSortings: Record<string, number>) {
    const entries = spellLists.value[listUuid].entries
    const promises: Promise<boolean>[] = []
    for (const presetUuid in newSortings) {
      const entry = entries.find((e) => e.uuid === presetUuid)
      if (entry !== undefined) {
        entry.sorting = newSortings[presetUuid]
        promises.push(DB.zauber.setSpellSorting(listUuid, presetUuid, newSortings[presetUuid]))
      }
    }
    entries.sort((a, b) => a.sorting - b.sorting)
    await Promise.all(promises)
  }

  async function moveSpell(
    sourceListUuid: string,
    targetListUuid: string,
    presetUuid: string,
    sorting: number
  ) {
    const source = spellLists.value[sourceListUuid]
    const target = spellLists.value[targetListUuid]
    const entry = source.entries.find((e) => e.uuid === presetUuid)
    if (entry === undefined) return
    source.entries = source.entries.filter((e) => e.uuid !== presetUuid)
    target.entries.push({ ...entry, sorting })
    target.entries.sort((a, b) => a.sorting - b.sorting)
    await DB.zauber.addSpellToList(targetListUuid, presetUuid, sorting)
    await DB.zauber.removeSpell(sourceListUuid, presetUuid)
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
    endConcentration,
    editSpellListName,
    deleteSpellList,
    removeSpell,
    addPreset,
    getPreset,
    deletePreset,
    addSpellToList,
    changeSpellListSorting,
    moveSpell
  }
})
