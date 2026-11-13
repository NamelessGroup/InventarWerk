<template>
  <div
    class="bg-primary-default my-2 grid rounded-sm border p-1"
    :class="{
      'border-accent': expanded,
      'border-primary-default': !expanded
    }"
    @click="expanded = !expanded"
  >
    <div class="flex items-center">
      <div class="break-wrap flex flex-1 items-center px-2">{{ spell.name }}</div>

      <button class="bg-primary-dark h-8 rounded-sm border-none px-3">
        <FontAwesomeIcon :icon="faWandSparkles" />
      </button>
      <TimedConfirmationButton
        v-if="canEdit"
        class="bg-primary-dark text-delete h-8 rounded-sm border-none px-3"
        :skip-confirmation="Settings.getInstance().noDeleteConfirmation"
        @confirm="deleteSpell()"
      >
        <FontAwesomeIcon :icon="faTrashCan" />

        <template #confirmation> Confirm </template>
      </TimedConfirmationButton>
    </div>

    <CollapseTransition>
      <div v-show="expanded">test</div>
    </CollapseTransition>
  </div>
</template>

<script setup lang="ts">
import type { SpellPreset } from '@/model/SpellPreset'
import { Settings } from '@/store/Settings'
import { ref, type PropType } from 'vue'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faWandSparkles, faTrashCan } from '@fortawesome/free-solid-svg-icons'

defineProps({
  spell: {
    type: Object as PropType<SpellPreset>,
    required: true
  },
  spellListUuid: {
    type: String,
    required: true
  },
  canEdit: {
    type: Boolean,
    default: false
  }
})

const expanded = ref(false)
</script>
