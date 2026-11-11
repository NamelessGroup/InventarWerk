<template>
  <PopUp @close="emit('close')">
    <div class="grid grid-cols-[auto_auto_1fr] gap-2">
      <span v-for="lvl in 9" :key="'l' + lvl" class="col-start-1" :style="{ gridRowStart: lvl }">
        Level {{ lvl }}:
      </span>
      <NumericInput
        v-for="lvl in 9"
        :key="'k' + lvl"
        v-model="values[lvl - 1]"
        :style="{ gridRowStart: lvl }"
        class="border-accent bg-primary-default col-start-2 rounded-sm border px-1 outline-hidden"
        @update:model-value="(v) => setLevel(lvl, v)"
      />
    </div>
  </PopUp>
</template>

<script setup lang="ts">
import PopUp from '../PopUp.vue'
import NumericInput from '../NumericInput.vue'
import { computed, type PropType } from 'vue'
import type { SpellSlots } from '@/model/SpellSlots.ts'
import { zauberData } from '@/store'

const props = defineProps({
  spellSlots: {
    type: Object as PropType<SpellSlots>,
    required: true
  },
  user: {
    type: String,
    required: true
  }
})

const emit = defineEmits(['close'])

const values = computed(() => Array.from({ length: 9 }, (_, i) => props.spellSlots[i + 1].length))

function setLevel(level: number, value: number) {
  zauberData().setSpellSlotMaxima(level, value, props.user)
}
</script>
