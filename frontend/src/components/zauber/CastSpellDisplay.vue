<!-- eslint-disable vue/no-v-html -->
<template>
  <PopUp :show-close-button="false" @emit="close()">
    <div class="flex h-full flex-col p-1 pb-0" @click="clear()">
      <div class="flex flex-1 flex-col gap-2 overflow-scroll text-sm">
        <div>Cast by: {{ cast.caster.name }}</div>
        <div class="text-lg">
          {{ cast.spell.name }}
          <span v-if="cast.castOptions.level === cast.spell.level">at level</span
          ><span v-else>upcast to level</span> {{ cast.castOptions.level }}
        </div>
        <div v-if="cast.castOptions.endedConcentrationOn" class="text-accent text-sm italic">
          Lost concentration on: {{ cast.castOptions.endedConcentrationOn.name }}
        </div>
        <div>
          <b>Time:</b> <span v-if="cast.castOptions.ritual">Ritual</span
          ><span>{{ cast.spell.time }}</span>
        </div>
        <div><b>Range:</b> {{ cast.spell.range }}</div>
        <div><b>Duration:</b> {{ cast.spell.duration }}</div>
        <div><b>Components:</b> {{ cast.spell.components.join(', ') }}</div>

        <b v-if="cast.spell.spellAttack">{{ cast.spell.spellAttack }}</b>

        <div v-if="cast.spell.savingThrow"><b>Saving Throw:</b> {{ cast.spell.savingThrow }}</div>
        <div v-if="cast.spell.abilityCheck"><b>Saving Throw:</b> {{ cast.spell.abilityCheck }}</div>
        <div v-if="cast.spell.conditionInflict !== ''">
          <b>Inflicts</b> {{ cast.spell.conditionInflict }}
        </div>
        <div v-if="immunities !== ''"><b>Grants Immunity to:</b> {{ immunities }}</div>
        <div v-if="cast.spell.damageResist !== ''">
          <b>Grants Resistance to:</b> {{ cast.spell.damageResist }}
        </div>
        <div v-if="cast.spell.affectsCreatureType">
          <b>Affects:</b> {{ cast.spell.affectsCreatureType }}
        </div>

        <button
          class="border-accent bg-primary-default rounded-sm border px-1 outline-hidden"
          @click="close()"
        >
          Close
        </button>

        <div class="marked" v-html="description"></div>
        <div class="marked" v-html="higherLevelDescription"></div>
      </div>

      <div class="bg-accent h-1" :style="{ width: barFill }"></div>
    </div>
  </PopUp>
</template>

<script setup lang="ts">
import { computed, onMounted, ref, type PropType } from 'vue'
import PopUp from '../PopUp.vue'
import type { CastSpell } from '@/model/CastSpell.ts'
import { marked } from 'marked'

const props = defineProps({
  showTime: {
    type: Number,
    default: 60000
  },
  cast: {
    type: Object as PropType<CastSpell>,
    required: true
  }
})

const emit = defineEmits(['close'])

const tickTime = 10

const intervalId = ref(-1)
const timeLived = ref(0)

const barFill = computed(() => `${100 - (timeLived.value / props.showTime) * 100}%`)

const immunities = computed(() => {
  return [props.cast.spell.damageImmune, props.cast.spell.conditionImmune]
    .filter((i) => i != '')
    .join(', ')
})

function clear() {
  clearInterval(intervalId.value)
}

function close() {
  emit('close')
}

const description = computed(() => marked.parse(props.cast.spell.description))
const higherLevelDescription = computed(() => marked.parse(props.cast.spell.higherLevel))

onMounted(() => {
  intervalId.value = setInterval(() => {
    if (timeLived.value > props.showTime) {
      close()
      return
    }
    timeLived.value += tickTime
  }, tickTime)
})
</script>
