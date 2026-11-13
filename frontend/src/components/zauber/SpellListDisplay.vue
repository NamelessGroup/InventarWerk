<template>
  <BaseContainer
    :container="spellList"
    :items="spellList.spells"
    @update-name="(n: string) => zauberData().editSpellListName(spellList.uuid, n)"
    @delete="zauberData().deleteSpellList(spellList.uuid)"
  >
    <template #item="{ item }">
      <SpellDisplay :spell="item" :can-edit="false" :spell-list-uuid="spellList.uuid" />
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
</template>

<script setup lang="ts">
import type { SpellList } from '@/model/SpellList'
import { zauberData } from '@/store'
import { ref, type PropType } from 'vue'
import SpellDisplay from './SpellDisplay.vue'

defineProps({
  spellList: {
    type: Object as PropType<SpellList>,
    required: true
  }
})

const showAddSpellPopup = ref(false)
</script>
