<template>
  <BaseContainer
    :container="spellList"
    :items="entries"
    drag-key="uuid"
    @update-name="(n: string) => zauberData().editSpellListName(spellList.uuid, n)"
    @delete="zauberData().deleteSpellList(spellList.uuid)"
    @move="moveSpell"
  >
    <template #extraInfoRight>
      <div class="shrink-0 text-sm opacity-60" title="Number of spells">
        {{ spellList.entries.length }}
      </div>
    </template>

    <template #item="{ item }">
      <div v-if="groupStarts.has(item.uuid)" class="mt-1 text-sm opacity-80">
        {{ item.level === 0 ? 'Cantrips' : `Level ${item.level}` }}
      </div>
      <SpellDisplay
        :spell="item"
        :can-delete="canDoStuff"
        :can-cast="canDoStuff"
        :can-drag="canDoStuff"
        @delete="zauberData().removeSpell(spellList.uuid, item.uuid)"
      />
    </template>

    <template #postItems>
      <button
        v-if="accountData().uuid === spellList.ownerUuid"
        class="bg-primary-default h-10 w-full rounded-sm text-center"
        @click="showAddSpellPopup = true"
      >
        + Add spell
      </button>
    </template>
  </BaseContainer>

  <AddSpellPreset
    v-if="showAddSpellPopup"
    :spell-list-uuid="spellList.uuid"
    @close="showAddSpellPopup = false"
  />
</template>

<script setup lang="ts">
import type { SpellEntry, SpellList } from '@/model/SpellList'
import type { DraggableItem } from '@/model/Drag'
import { accountData, zauberData } from '@/store'
import { Settings } from '@/store/Settings'
import { computed, ref, type PropType } from 'vue'
import SpellDisplay from './SpellDisplay.vue'
import BaseContainer from '../BaseContainer.vue'
import AddSpellPreset from './AddSpellPreset.vue'

const props = defineProps({
  spellList: {
    type: Object as PropType<SpellList>,
    required: true
  }
})

const showAddSpellPopup = ref(false)

const entries = computed(() => {
  if (!Settings.getInstance().groupSpellsByLevel) return props.spellList.entries
  // sort is stable, so the order within a level is kept
  return [...props.spellList.entries].sort((a, b) => a.level - b.level)
})

const groupStarts = computed(() => {
  const starts = new Set<string>()
  if (!Settings.getInstance().groupSpellsByLevel) return starts
  let lastLevel: number | undefined
  for (const entry of entries.value) {
    if (entry.level !== lastLevel) starts.add(entry.uuid)
    lastLevel = entry.level
  }
  return starts
})

const canDoStuff = computed(() => props.spellList.ownerUuid === accountData().uuid)

async function moveSpell(item: DraggableItem<SpellEntry>, newIndex: number, movedHere: boolean) {
  if (!canDoStuff.value) {
    throw new Error('You can only move spells into your own spell lists!')
  }
  if (movedHere) {
    const source = zauberData().spellLists[item.sourceContainer]
    if (source?.ownerUuid !== accountData().uuid) {
      throw new Error('You can only move spells out of your own spell lists!')
    }
    if (props.spellList.entries.some((e) => e.uuid === item.uuid)) {
      throw new Error(`This spell list already contains ${item.name}!`)
    }
  }

  const order = entries.value.map((e) => e.uuid).filter((uuid) => uuid !== item.uuid)
  order.splice(newIndex, 0, item.uuid)
  const oldSortings = new Map(props.spellList.entries.map((e) => [e.uuid, e.sorting]))

  if (movedHere) {
    await zauberData().moveSpell(item.sourceContainer, props.spellList.uuid, item.uuid, newIndex)
  }

  const sortingsToUpdate: Record<string, number> = {}
  order.forEach((uuid, idx) => {
    if (movedHere && uuid === item.uuid) return
    if (oldSortings.get(uuid) !== idx) sortingsToUpdate[uuid] = idx
  })
  if (Object.keys(sortingsToUpdate).length > 0) {
    await zauberData().changeSpellListSorting(props.spellList.uuid, sortingsToUpdate)
  }
}
</script>
