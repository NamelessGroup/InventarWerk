<template>
  <BaseContainer
    :container="spellList"
    :items="spellList.entries"
    drag-key="uuid"
    @update-name="(n: string) => zauberData().editSpellListName(spellList.uuid, n)"
    @delete="zauberData().deleteSpellList(spellList.uuid)"
  >
    <template #item="{ item }">
      <SpellDisplay
        :spell="item"
        :can-delete="canDoStuff"
        :can-cast="canDoStuff"
        @delete="zauberData().removeSpell(spellList.uuid, item.uuid)"
      />
    </template>

    <template #postItems>
      <button
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
import type { SpellList } from '@/model/SpellList'
import { accountData, zauberData } from '@/store'
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

const canDoStuff = computed(() => props.spellList.ownerUuid === accountData().uuid)
</script>
