<template>
  <BaseContainer
    :container="spellList"
    :items="entries"
    :groups="groups"
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
import type { DraggableItem, ItemGroup } from '@/model/Drag'
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

const groups = computed<ItemGroup<SpellEntry>[] | undefined>(() => {
  if (!Settings.getInstance().groupSpellsByLevel) return undefined
  const result: ItemGroup<SpellEntry>[] = []
  for (const entry of entries.value) {
    const last = result[result.length - 1]
    if (last?.key === entry.level.toString()) {
      last.items.push(entry)
    } else {
      result.push({ key: entry.level.toString(), label: levelLabel(entry.level), items: [entry] })
    }
  }
  return result
})

function levelLabel(level: number) {
  return level === 0 ? 'Cantrips' : `Level ${level}`
}

const canDoStuff = computed(() => props.spellList.ownerUuid === accountData().uuid)

async function moveSpell(
  item: DraggableItem<SpellEntry>,
  newIndex: number,
  movedHere: boolean,
  groupKey: string
) {
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

  const others = entries.value.filter((e) => e.uuid !== item.uuid)
  let index = newIndex
  if (groups.value !== undefined) {
    const offset = others.filter((e) => e.level < item.level).length
    const sameLevelCount = others.filter((e) => e.level === item.level).length
    const droppedLevel = Number(groupKey)
    if (groupKey === item.level.toString()) {
      index = offset + newIndex
    } else if (groupKey !== '' && droppedLevel < item.level) {
      index = offset
    } else {
      index = offset + sameLevelCount
    }
  }
  const order = others.map((e) => e.uuid)
  order.splice(index, 0, item.uuid)
  const oldSortings = new Map(props.spellList.entries.map((e) => [e.uuid, e.sorting]))

  if (movedHere) {
    await zauberData().moveSpell(item.sourceContainer, props.spellList.uuid, item.uuid, index)
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
