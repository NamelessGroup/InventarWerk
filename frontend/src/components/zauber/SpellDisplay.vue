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
      <div v-if="canDrag" class="drag-handle mx-1 h-fit cursor-grab" @click.stop>⋮⋮</div>

      <div class="flex flex-1 flex-col gap-1">
        <div class="break-wrap">{{ spell.name }}</div>
        <div v-if="tags.length > 0" class="flex flex-wrap gap-1 text-xs">
          <span
            v-for="tag in tags"
            :key="tag.label"
            class="bg-primary-dark rounded-sm px-1.5"
            :class="tag.highlight ? 'text-accent' : 'opacity-80'"
            :title="tag.title"
            >{{ tag.label }}</span
          >
        </div>
      </div>

      <button
        v-if="canCast"
        class="bg-primary-dark text-accent h-8 rounded-sm border-none px-3"
        @click="cast"
      >
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
        <div><b>Source:</b> {{ spell.source }}</div>
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

  <CastSpellPopUp v-if="showCastPopUp" :spell="spell" @close="showCastPopUp = false" />
</template>

<script setup lang="ts">
import { Settings, type SpellTag } from '@/store/Settings'
import { computed, ref, type PropType } from 'vue'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faWandSparkles, faTrashCan } from '@fortawesome/free-solid-svg-icons'
import type { SpellEntry } from '@/model/SpellList'
import CollapseTransition from '../CollapseTransition.vue'
import TimedConfirmationButton from '../TimedConfirmationButton.vue'
import { marked } from 'marked'
import CastSpellPopUp from './CastSpellPopUp.vue'

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
  },
  canDrag: {
    type: Boolean,
    default: false
  }
})

const expanded = ref(false)

const emit = defineEmits(['delete'])

const showCastPopUp = ref(false)

interface Tag {
  key: SpellTag
  label: string
  title: string
  highlight?: boolean
}

const tags = computed(() => {
  const result: Tag[] = [
    props.spell.level === 0
      ? { key: 'level', label: 'Cantrip', title: 'Cantrip' }
      : { key: 'level', label: `Lvl ${props.spell.level}`, title: `Level ${props.spell.level}` }
  ]
  // components are stored as e.g. "Verbal", "Somatic", "Material (a feather)"
  const componentLetters = props.spell.components
    .map((c) => c.charAt(0).toUpperCase())
    .filter((c) => ['V', 'S', 'M'].includes(c))
  if (componentLetters.length > 0) {
    result.push({
      key: 'components',
      label: componentLetters.join(' '),
      title: props.spell.components.join(', ')
    })
  }
  if (props.spell.concentration) {
    result.push({
      key: 'concentration',
      label: 'Concentration',
      title: 'Requires concentration',
      highlight: true
    })
  }
  if (props.spell.ritual) {
    result.push({
      key: 'ritual',
      label: 'Ritual',
      title: 'Can be cast as a ritual',
      highlight: true
    })
  }
  return result.filter((tag) => Settings.getInstance().showSpellTag(tag.key))
})

const description = computed(() => marked.parse(props.spell.description))
const higherLevelDescription = computed(() => marked.parse(props.spell.higherLevel))

function cast(e: Event) {
  e.stopPropagation()
  showCastPopUp.value = true
}
</script>
