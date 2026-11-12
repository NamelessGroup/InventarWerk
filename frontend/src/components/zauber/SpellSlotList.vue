<template>
  <div
    class="border-accent bg-primary-dark flex flex-col gap-2 overflow-hidden rounded-sm border-2 p-2"
  >
    <div class="flex items-center gap-2 text-lg">
      <DiscordImage :user="owner" class="h-6" />
      <span v-if="accountData().userIsDm">{{ owner.name }}</span>
      <span>Spell Slots</span>
      <div class="flex-1"></div>
      <button @click="temp()">temp</button>
      <button
        v-if="accountData().userIsDm"
        class="border-accent bg-primary-default mr-2 flex h-7 w-7 shrink-0 items-center justify-center rounded-sm border text-sm"
        @click="zauberData().resetSpellSlots()"
      >
        <FontAwesomeIcon :icon="faRotateLeft" />
      </button>
      <button
        class="border-accent bg-primary-default mr-2 flex h-7 w-7 shrink-0 items-center justify-center rounded-sm border text-sm"
        @click="showManageMaxima = true"
      >
        <FontAwesomeIcon :icon="faGears" />
      </button>
    </div>

    <div class="flex flex-wrap gap-5">
      <div v-for="lvl in maxSpellSlot" :key="lvl" class="flex items-center gap-1">
        <span>{{ lvl }}:</span>
        <SpellSlotDot v-for="[idx, slot] in spellSlots[lvl].entries()" :key="idx" :filled="!slot" />
      </div>
    </div>

    <div v-if="concentrationSpell" class="text-lg">
      Concentrating on <span class="bold text-accent italic">{{ concentrationSpell }}</span>
      <button
        class="border-accent bg-primary-default mr-2 flex h-7 w-fit shrink-0 items-center gap-2 rounded-sm border"
      >
        <FontAwesomeIcon :icon="faX" />
        <span>Cancel Concentration</span>
      </button>
    </div>

    <ManageSlotMaximaPopUp
      v-if="showManageMaxima"
      :spell-slots="spellSlots"
      :user="user"
      @close="showManageMaxima = false"
    />
  </div>
</template>

<script setup lang="ts">
import type { SpellSlots } from '@/model/SpellSlots'
import { accountData, zauberData } from '@/store'
import { faGears, faRotateLeft, faX } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { computed, ref, type PropType } from 'vue'
import DiscordImage from '../DiscordImage.vue'
import ManageSlotMaximaPopUp from './ManageSlotMaximaPopUp.vue'
import SpellSlotDot from './SpellSlotDot.vue'

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
  }
})

const owner = computed(
  () =>
    accountData().accounts.filter((account) => account.uuid === props.user)[0] ?? {
      name: 'Unknown',
      avatar: null,
      dm: false,
      uuid: ''
    }
)

const maxSpellSlot = computed(() => {
  let max = 0
  for (let i = 1; i <= 9; i++) {
    if (props.spellSlots[i] !== undefined && props.spellSlots[i].length > 0) {
      max = i
    }
  }
  return max
})

function temp() {
  zauberData().setAvailableSlots(1, 3)
}
</script>
