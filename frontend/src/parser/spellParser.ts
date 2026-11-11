import type { SpellPreset } from "@/model/SpellPreset"

interface ToolsSpell {
  name: string
  source: string
  level: number
  school: string,
  time: Time[],
  range: Range,
  components:Components,
  duration: Duration[],
  entries: string[],
  entriesHigherLevel: HigherLevel,
}

interface Time {
  number: number
  unit: string
}

interface Range {
  type: string
  distance: {
    type: string
    amount: number
  }
}

interface Components {
  v: boolean
  s: boolean
  m: string|undefined
}

interface Duration {
  type: string
  duration: {
    type: string,
    amount: number
  } | undefined
}

interface HigherLevel {
  type: 'entries'
  name: string,
  entries: string[]
}

function convertSpell(original: ToolsSpell): SpellPreset {
  return {
    
  }
}
