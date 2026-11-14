<template>
  <PopUp @close="emit('close')">
    <div class="flex flex-col gap-3">
      <span class="text-xl">Cast {{ spell.name }}</span>
      <div v-if="currentConcentration !== null && spell.concentration" class="text-amber">
        You will loose concentration on {{ currentConcentration.name }}
      </div>
      <div v-if="spell.ritual" class="flex items-center gap-x-2">
        <input id="ritual" v-model="ritual" type="checkbox" />
        <label for="ritual">Cast as Ritual</label>
      </div>

      <div v-if="spell.level > 0" class="flex items-center gap-x-2">
        <label for="level">Cast at level:</label>
        <select
          id="level"
          v-model.number="level"
          class="border-accent bg-primary-default rounded-sm border px-1 outline-hidden"
        >
          <option v-for="lvl in availableLevel" :key="lvl" :value="lvl.toString()">
            {{ lvl }}
          </option>
        </select>
      </div>
      <button
        class="border-accent bg-primary-default w-full rounded-sm border p-1 outline-hidden"
        @click="cast()"
      >
        Cast
      </button>
      <div class="text-error-text">{{ errorText }}</div>
    </div>
  </PopUp>
</template>

<script setup lang="ts">
import { computed, ref, type PropType } from 'vue'
import PopUp from '../PopUp.vue'
import type { SpellPreset } from '@/model/SpellPreset.ts'
import { accountData, zauberData } from '@/store/index.ts'
import { SpellSlotState } from '@/model/SpellSlots.ts'
import { DB } from '@/store/db/index.ts'

const props = defineProps({
  spell: {
    type: Object as PropType<SpellPreset>,
    required: true
  }
})

const emit = defineEmits(['close'])

const ritual = ref(false)
const level = ref(props.spell.level)
const errorText = ref('')
const currentConcentration = computed(() => {
  const conUuid = zauberData().concentrationSpell[accountData().uuid]
  if (conUuid === undefined || conUuid === null) return null
  return zauberData().getPreset(conUuid)
})

const availableLevel = computed(() =>
  Array.from({ length: 9 - props.spell.level + 1 }, (_, i) => i + props.spell.level)
)

async function cast() {
  let slotAvailable = true
  const slots = zauberData().spellSlots[accountData().uuid]
  if (slots[level.value] === undefined || slots[level.value].length === 0) slotAvailable = false
  else if (!slots[level.value].some((s) => s === SpellSlotState.AVAILABLE)) slotAvailable = false
  if (!slotAvailable && !ritual.value) {
    errorText.value = 'No spell slot available'
    return
  }
  for (let i = 0; i < slots[level.value].length; i++) {
    if (slots[level.value][i] === SpellSlotState.AVAILABLE) {
      slots[level.value][i] = SpellSlotState.USED
      break
    }
  }
  if (props.spell.concentration) {
    zauberData().concentrationSpell[accountData().uuid] = props.spell.uuid
  }

  await DB.zauber.cast(props.spell.uuid, level.value, ritual.value)
  emit('close')
}
</script>
