<template>
  <div
    class="border-accent bg-primary-dark flex flex-col overflow-hidden rounded-sm border-2 p-2"
    :class="expanded ? 'w-full' : 'w-fit'"
  >
    <div class="flex items-center gap-2 text-lg">
      <DiscordImage :user="owner" class="h-6" />
      <span v-if="accountData().userIsDm">{{ owner.name }}</span>
      <span v-if="expanded">Spell Slots</span>
      <span v-if="!expanded && concentrationSpell" class="text-accent font-bold italic">C</span>
      <div class="flex-1"></div>
      <button
        class="border-accent bg-primary-default h-7 w-7 shrink-0 rounded-sm border"
        @click="expanded = !expanded"
      >
        <FontAwesomeIcon
          class="transition"
          :class="{ 'rotate-180': !expanded }"
          :icon="faChevronUp"
        />
      </button>
      <button
        v-if="canEdit && expanded"
        class="border-accent bg-primary-default flex h-7 w-7 shrink-0 items-center justify-center rounded-sm border text-sm"
        @click="zauberData().resetSpellSlots(user)"
      >
        <FontAwesomeIcon :icon="faRotateLeft" />
      </button>
      <button
        v-if="expanded"
        class="border-accent bg-primary-default flex h-7 w-7 shrink-0 items-center justify-center rounded-sm border text-sm"
        @click="showManageMaxima = true"
      >
        <FontAwesomeIcon :icon="faGears" />
      </button>
    </div>
    <CollapseTransition with-opacity speed="0.5s">
      <div v-if="expanded" class="mt-2 flex flex-col gap-2">
        <div class="flex flex-wrap gap-5">
          <div v-for="lvl in maxSpellSlot" :key="lvl" class="flex items-center gap-1">
            <span>{{ lvl }}:</span>
            <SpellSlotDot
              v-for="[idx, slot] in spellSlots[lvl].entries()"
              :key="idx"
              :state="slot"
              :clickable="canEdit"
              @click="toggleSlot(lvl, slot)"
            />
          </div>
        </div>

        <div v-if="concentrationSpell" class="flex items-center gap-2">
          <span
            >Concentrating on
            <span class="bold text-accent italic">{{
              zauberData().getPreset(concentrationSpell).name
            }}</span></span
          >
          <button
            v-if="canEdit"
            class="border-accent bg-primary-default mr-2 flex h-7 w-fit shrink-0 items-center gap-2 rounded-sm border"
            @click="zauberData().endConcentration(user)"
          >
            <FontAwesomeIcon :icon="faX" />
            <span>Cancel Concentration</span>
          </button>
        </div>
      </div>
    </CollapseTransition>

    <ManageSlotMaximaPopUp
      v-if="showManageMaxima"
      :spell-slots="spellSlots"
      :user="user"
      @close="showManageMaxima = false"
    />
  </div>
</template>

<script setup lang="ts">
import { SpellSlotState, type SpellSlots } from '@/model/SpellSlots'
import { accountData, zauberData } from '@/store'
import { faChevronUp, faGears, faRotateLeft, faX } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { computed, ref, type PropType } from 'vue'
import DiscordImage from '../DiscordImage.vue'
import ManageSlotMaximaPopUp from './ManageSlotMaximaPopUp.vue'
import SpellSlotDot from './SpellSlotDot.vue'
import CollapseTransition from '../CollapseTransition.vue'

const showManageMaxima = ref(false)

const props = defineProps({
  spellSlots: {
    type: Object as PropType<SpellSlots>,
    required: true
  },
  user: {
    type: String,
    required: true
  },
  concentrationSpell: {
    type: String,
    required: false,
    default: undefined
  },
  defaultExpanded: {
    type: Boolean,
    default: true
  }
})

const expanded = ref(props.defaultExpanded)

const owner = computed(
  () =>
    accountData().accounts.filter((account) => account.uuid === props.user)[0] ?? {
      name: 'Unknown',
      avatar: null,
      dm: false,
      uuid: ''
    }
)

const canEdit = computed(() => accountData().userIsDm || accountData().uuid === props.user)

function toggleSlot(level: number, state: SpellSlotState) {
  if (!canEdit.value) return
  const available = props.spellSlots[level].filter((s) => s === SpellSlotState.AVAILABLE).length
  const newAvailable = state === SpellSlotState.AVAILABLE ? available - 1 : available + 1
  zauberData().setAvailableSlots(level, newAvailable, props.user)
}

const maxSpellSlot = computed(() => {
  let max = 0
  for (let i = 1; i <= 9; i++) {
    if (props.spellSlots[i] !== undefined && props.spellSlots[i].length > 0) {
      max = i
    }
  }
  return max
})
</script>
