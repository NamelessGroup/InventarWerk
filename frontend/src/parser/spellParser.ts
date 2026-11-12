import type { SpellPreset } from '@/model/SpellPreset'
import { parse, type Entry } from './Entry'
import { processList } from './DB'

interface ToolsSpell {
  name: string
  source: string
  level: number
  school: string
  time: Time[]
  range: Range
  components: Components
  duration: Duration[]
  entries: Entry[]
  entriesHigherLevel?: Entry[]
  conditionInflict?: string[]
  savingThrow?: string[]
  affectsCreatureType?: string[]
  damageInflict?: string[]
  spellAttack?: string[]
  damageImmune?: string[]
  abilityCheck?: string[]
  damageResist?: string[]
  conditionImmune?: string[]
  meta: { ritual: boolean } | undefined
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
  m: string | undefined
}

interface Duration {
  type: string
  duration:
    | {
        type: string
        amount: number
      }
    | undefined
  concentration: boolean | undefined
}

type SourceJson = Record<string, SourceBookData>
type SourceBookData = Record<string, SourceData>
interface SourceData {
  class: {
    name: string
    source: string
  }[]
}

function parseComponents(c: Components) {
  const comps: string[] = []
  if (c.v) comps.push('Verbal')
  if (c.s) comps.push('Somantic')
  if (c.m) comps.push(`Material (${c.m})`)
  return comps
}

function array(a: string[] | undefined) {
  if (!a) return ''
  return a.join(', ')
}

function parseAttack(a: string[] | undefined) {
  if (!a) return ''
  const la = a?.map((b) => b.toLowerCase())
  const r: string[] = []
  if (la.includes('r')) r.push('Ranged')
  if (la.includes('m')) r.push('Meele')
  return r.join(', ')
}

function convertSpell(original: ToolsSpell, source?: SourceData): SpellPreset {
  return {
    uuid: '',
    name: original.name,
    duration: original.duration
      .map((d) => (d.duration ? `${d.duration.amount} ${d.duration.type}` : d.type))
      .join(', '),
    range: [`${original.range.distance} ${original.range.type}`],
    description: original.entries.map(parse).join('\n\n'),
    higherLevel: original.entriesHigherLevel
      ? original.entriesHigherLevel.map(parse).join('\n\n')
      : '',
    components: parseComponents(original.components),
    concentration: original.duration.some((t) => t.concentration === true),
    level: original.level,
    school: ['TODO'],
    conditionInflict: array(original.conditionInflict),
    conditionImmune: array(original.conditionImmune),
    savingThrow: array(original.savingThrow),
    affectsCreatureType: array(original.affectsCreatureType),
    damageInflict: array(original.damageInflict),
    damageImmune: array(original.damageImmune),
    abilityCheck: array(original.abilityCheck),
    damageResist: array(original.damageResist),
    time: original.time.map((t) => `${t.number} ${t.unit}`).join(', '),

    spellAttack: parseAttack(original.spellAttack),

    ritual: original.meta?.ritual ?? false,

    class: source ? source.class.map((c) => c.name) : [],
    feats: [],
    species: [],
    subClass: []
  }
}

interface SpellJson {
  spells: ToolsSpell[]
}

function flattenSources(sources: SourceJson): SourceBookData {
  const result: SourceBookData = {}
  const books = Object.keys(sources)
  for (const book of books) {
    const data = sources[book]
    const spells = Object.keys(data)
    for (const spell of spells) {
      if (result[spell] !== undefined) {
        result[spell].class.push(...data[spell].class)
      } else {
        result[spell] = data[spell]
      }
    }
  }
  return result
}

export function parseSpellList(spellList: SpellJson, sources: SourceJson) {
  const flattenedSources = flattenSources(sources)
  const spells = spellList.spells.map((s) => convertSpell(s, flattenedSources[s.name]))

  processList(spells, 'TODO', 'TODO')
}
