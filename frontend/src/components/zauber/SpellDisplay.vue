<!-- eslint-disable vue/no-v-html -->
<template>
  <div
    class="bg-primary-default grid rounded-sm border p-1"
    :class="{
      'border-accent': expanded,
      'border-primary-default': !expanded
    }"
    @click="expanded = !expanded"
  >
    <div class="flex items-center gap-2">
      <div class="break-wrap flex flex-1 items-center">{{ spell.name }}</div>

      <button v-if="canCast" class="bg-primary-dark text-accent h-8 rounded-sm border-none px-3">
        <FontAwesomeIcon :icon="faWandSparkles" />
      </button>
      <TimedConfirmationButton
        v-if="canDelete"
        class="bg-primary-dark text-delete h-8 rounded-sm border-none px-3"
        :skip-confirmation="Settings.getInstance().noDeleteConfirmation"
        @confirm="emit('delete')"
      >
        <FontAwesomeIcon :icon="faTrashCan" />

        <template #confirmation> Confirm </template>
      </TimedConfirmationButton>
    </div>

    <CollapseTransition>
      <div v-show="expanded" class="flex flex-col gap-1 text-xs">
        <i
          ><span v-if="spell.level === 0">{{ spell.school }} Cantrip</span>
          <span v-else>Level {{ spell.level }} {{ spell.school }}</span></i
        >
        <div><b>Time:</b> {{ spell.time }}<span v-if="spell.ritual"> or Ritual</span></div>
        <div><b>Range:</b> {{ spell.range }}</div>
        <div>
          <b>Duration:</b> {{ spell.duration
          }}<span v-if="spell.concentration"> (Concentration)</span>
        </div>
        <div><b>Components:</b> {{ spell.components.join(', ') }}</div>
        <div class="marked" v-html="description"></div>
        <div class="marked" v-html="higherLevelDescription"></div>
      </div>
    </CollapseTransition>
  </div>
</template>

<script setup lang="ts">
import { Settings } from '@/store/Settings'
import { computed, ref, type PropType } from 'vue'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faWandSparkles, faTrashCan } from '@fortawesome/free-solid-svg-icons'
import type { SpellEntry } from '@/model/SpellList'
import CollapseTransition from '../CollapseTransition.vue'
import TimedConfirmationButton from '../TimedConfirmationButton.vue'
import { marked } from 'marked'

const props = defineProps({
  spell: {
    type: Object as PropType<SpellEntry>,
    required: true
  },
  canDelete: {
    type: Boolean,
    default: false
  },
  canCast: {
    type: Boolean,
    default: false
  }
})

const expanded = ref(false)

const emit = defineEmits(['delete'])

const description = computed(() => marked.parse(props.spell.description))
const higherLevelDescription = computed(() => marked.parse(props.spell.higherLevel))
</script>
