<template>
  <div>
    <CastSpellDisplay
      v-for="(c, i) in castQueue"
      :key="c.caster.uuid + c.spell.uuid"
      :cast="c"
      @close="castQueue.splice(i, 1)"
    />
  </div>
</template>

<script setup lang="ts">
import type { CastSpell } from '@/model/CastSpell'
import { DB } from '@/store/db'
import { ref } from 'vue'
import CastSpellDisplay from './CastSpellDisplay.vue'
import { accountData, zauberData } from '@/store'

const castQueue = ref<CastSpell[]>([])

DB.cast.addCastListener((a, s, l, r, c) =>
  castQueue.value.push({
    caster: accountData().accounts.filter((ac) => ac.uuid === a)[0],
    spell: s,
    castOptions: {
      level: l,
      ritual: r,
      endedConcentrationOn: c ? zauberData().getPreset(c) : undefined
    }
  })
)
</script>
