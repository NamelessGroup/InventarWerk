import { ErrorHandler } from '@/errorHandling/ErrorHandler'
import type { ItemPreset } from '@/model/ItemPreset'
import { DatabaseHandler } from '@/store/DatabaseHandler'
import axios from 'axios'

interface ItemJSON {
  name: string
  source?: string
  page?: number
  reqAttune?: string | boolean
  weight?: number
  entries: Entry[]
  value?: number
  type?: string
  ac?: number
  ammoType?: string
  dmg1?: string
  dmg2?: string
  dmgType?: string
  mastery?: string[]
  property?: string[]
  range?: string
  reload?: string
  stealth?: boolean
  strength?: number
  weaponCategory?: string
}
interface ItemListJSON {
  baseitem?: ItemJSON[]
  item?: ItemJSON[]
}
type Entry = string | ComplexEntry
interface ComplexEntry {
  type: LineType
  name?: string
  entries?: Entry[]
  caption?: string
  colLabels?: string[]
  rows?: string[][]
  items?: string[]
}

const typeTranslator: Record<string, string> = {
  undefined: 'Other',
  '$A|DMG': 'Treasure (art object)',
  '$A|XDMG': 'Treasure (art object)',
  $C: 'Treasure (coinage)',
  '$G|DMG': 'Treasure (gemstone)',
  '$G|XDMG': 'Treasure (gemstone)',
  A: 'Ammunition',
  'AF|DMG': 'Ammunition (futuristic)',
  'AF|XDMG': 'Ammunition (futuristic)',
  'AIR|DMG': 'Vehicle (air)',
  'AIR|XPHB': 'Vehicle (air)',
  AT: "Artisan's tools",
  'AT|XPHB': "Artisan's tools",
  'A|XPHB': 'Ammunition',
  'EXP|DMG': 'Explosive',
  'EXP|XDMG': 'Explosive',
  FD: 'Food and drink',
  'FD|XPHB': 'Food and drink',
  G: 'Adventuring gear',
  GS: 'Gaming set',
  'GS|XPHB': 'Gaming set',
  'G|XPHB': 'Adventuring gear',
  HA: 'Heavy armor',
  'HA|XPHB': 'Heavy armor',
  'IDG|TDCSR': 'Illegal drug',
  INS: 'Instrument',
  'INS|XPHB': 'Instrument',
  LA: 'Light armor',
  'LA|XPHB': 'Light armor',
  M: 'Martial weapon',
  MA: 'Medium armor',
  'MA|XPHB': 'Medium armor',
  MNT: 'Mount',
  'MNT|XPHB': 'Mount',
  'M|XPHB': 'Martial weapon',
  OTH: 'Other',
  P: 'Potion',
  'P|XPHB': 'Potion',
  R: 'Ranged weapon',
  'RD|DMG': 'Rod',
  'RD|XDMG': 'Rod',
  'RG|DMG': 'Ring',
  'RG|XDMG': 'Ring',
  'R|XPHB': 'Ranged weapon',
  S: 'Shield',
  SCF: 'Spellcasting focus',
  'SCF|XPHB': 'Spellcasting focus',
  'SC|DMG': 'Scroll',
  'SC|XPHB': 'Scroll',
  SHP: 'Vehicle (water)',
  'SHP|XPHB': 'Vehicle (water)',
  'SPC|AAG': 'Vehicle (space)',
  'S|XPHB': 'Shield',
  T: 'Tools',
  TAH: 'Tack and harness',
  'TAH|XPHB': 'Tack and harness',
  'TB|XDMG': 'Trade bar',
  TG: 'Trade good',
  'TG|XDMG': 'Trade good',
  'T|XPHB': 'Tools',
  VEH: 'Vehicle (land)',
  'VEH|XPHB': 'Vehicle (land)',
  'WD|DMG': 'Wand',
  'WD|XDMG': 'Wand'
}

const dmgTypeTranslator: Record<string, string> = {
  N: 'Necrotic', //"Antimatter Rifle"
  P: 'Piercing', //"Yklwa",
  S: 'Slashing', //"Whip",
  B: 'Bludgeoning', //"Wooden Staff",
  R: 'Radiant' //"Laser Rifle"
}

const propTranslator: Record<string, string> = {
  "AF|DMG": "Ammunition", // shotgun
  "2H": "Two handed",
  "AF|XDMG": "Ammunition",
  "RLD": "Reload",
  "RLD|XDMG": "Reload",
  "2H|XPHB": "Two handed",
  "BF|DMG": "Burst fire", //Automatic rifle
  "BF|XDMG": "Burst fire",
  "V": "Versatile",
  "V|XPHB": "Versatile",
  "A": "Ammuntition",
  "LD": "Loading",
  "A|XPHB": "Ammuntition",
  "LD|XPHB": "Loading",
  "L": "Light",
  "L|XPHB": "Light",
  "F": "Finesse",
  "T": "Thrown",
  "F|XPHB": "Finesse",
  "T|XPHB": "Thrown",
  "S": "Special",
  "H": "Heavy",
  "R": "Reach",
  "H|XPHB": "Heavy",
  "R|XPHB": "Reach"
}

type LineType = 'entries' | 'inset' | 'list' | 'section' | 'table' | 'quote'

const descriptionTranslator: Record<LineType, (entry: ComplexEntry, section?: boolean) => string> =
  {
    entries: entryParser, //"Ythryn Mythallar",
    inset: insetParser, //"Will of the Talon (Dormant)",
    list: listParser, //"Ythryn Mythallar",
    section: sectionParser, //"Keystone of Creation",
    table: tableParser, //"Xen'drik Trinket"
    quote: quoteParser //Iggwilv's Cauldron
  }

function quoteParser(entry: ComplexEntry) {
  const lines: Array<string> = []
  for (const line of entry.entries ?? '') {
    if (typeof line === 'string') {
      lines.push(line)
    } else {
      const complexEntry: ComplexEntry = line
      lines.push(descriptionTranslator[line.type](complexEntry))
    }
  }
  lines[0] = `*"${lines[0]}`
  lines[lines.length - 1] = `${lines[lines.length - 1]}"*`
  return lines.join('\n')
}

function entryParser(entry: ComplexEntry, section?: boolean) {
  const lines: Array<string> = []
  //Should at least have one string line
  for (const line of entry.entries ?? '') {
    if (typeof line === 'string') {
      lines.push(line)
    } else {
      const complexEntry: ComplexEntry = line
      lines.push(descriptionTranslator[line.type](complexEntry))
    }
  }
  if (lines.length == 0) lines.push('')
  if ((section ?? false) == true && entry.name) {
    lines.unshift(`## ${entry.name}`)
  } else {
    lines[0] = `${entry.name ? '**' + entry.name + '**. ' : ''}${lines[0]}`
  }
  return lines.join('\n\n')
}

function insetParser(entry: ComplexEntry) {
  const lines: Array<string> = ['---', `**${entry.name ?? ''}**`]
  for (const line of entry.entries ?? '') {
    if (typeof line === 'string') {
      lines.push(line)
    } else {
      const complexEntry: ComplexEntry = line
      lines.push(descriptionTranslator[line.type](complexEntry))
    }
  }
  lines.push('---')
  return lines.join('\n\n')
}

function listParser(entry: ComplexEntry) {
  const lines: Array<string> = []
  for (const line of entry.items ?? '') {
    lines.push(`- ${line}`)
  }
  return lines.join('\n')
}

function sectionParser(entry: ComplexEntry) {
  const lines: Array<string> = []
  for (const line of entry.entries ?? '') {
    if (typeof line === 'string') {
      lines.push(line)
    } else {
      const complexEntry: ComplexEntry = line
      if (line.type == 'entries') lines.push(descriptionTranslator[line.type](complexEntry, true))
      else lines.push(descriptionTranslator[line.type](complexEntry))
    }
  }
  return lines.join('\n\n')
}

function tableParser(entry: ComplexEntry) {
  const lines: Array<string> = [
    ` *${entry.caption ?? ''}*`,
    `|${entry.colLabels?.join('|')}|`,
    `|${entry.colLabels?.map(() => '---').join('|')}|`
  ]
  for (const row of entry.rows ?? []) {
    lines.push(`|${row.join('|')}|`)
  }
  return lines.join('\n')
}

export async function parseItems(itemList: ItemListJSON) {
  const parsedItemList: Array<ItemPreset> = []

  const regex1 = /\{@[^\s|}]+ ([^|}]+)}/g

  const regex2 = /\{@[^\s|}]+ ([^|}]+)\|[^|}]+}/g

  const regex3 = /\{@[^\s|}]+ [^|}]+\|[^|}]+\|([^|}]+)}/g

  const joinedItems = [...(itemList.baseitem ?? []), ...(itemList.item ?? [])]

  for (const x of joinedItems) {
    const parsedItem: ItemPreset = {
      name: '',
      uuid: '',
      description: '',
      price: 0,
      creator: 'public-import',
      itemType: '',
      weight: 0
    }
    parsedItem.name = x.name
    if (x.source) parsedItem.name += ` (${x.source})`
    parsedItem.price = Math.round(x.value ?? 0)
    parsedItem.weight = x.weight ?? 0
    parsedItem.itemType =
      typeTranslator[x.type ?? 'undefined'] ??
      (() => {
        console.info(`Missing Type: ${x.type} on item ${x.name}`)
        return "Other"
      })()
    const lines: Array<string> = []
    for (const line of x.entries ?? '') {
      if (typeof line === 'string') {
        lines.push(line)
      } else {
        const complexEntry: ComplexEntry = line
        lines.push(descriptionTranslator[line.type](complexEntry))
      }
    }
    if (x.reqAttune) {
      if (x.reqAttune === true) parsedItem.description = `*Requires Attunement*\n\n`
      else parsedItem.description = `*Requires Attunement ${x.reqAttune}*\n\n`
    }
    if (x.ac) {
      parsedItem.description += `Armor Class: ${x.ac}\n\n`
    }
    if (x.ammoType) {
      parsedItem.description += `Ammotype: ${x.ammoType}\n\n`
    }

    if (x.dmg1) {
      parsedItem.description += `Damage: ${x.dmg1}`
      if (x.dmg2) {
        parsedItem.description += `/${x.dmg2}`
      }
      if (x.dmgType) {
        parsedItem.description += ` ${dmgTypeTranslator[x.dmgType]} Damage\n`
      }
      parsedItem.description += `\n`
    }
    if (x.property) {
      parsedItem.description += `Properties: \n`
      for (const p of x.property) {
        parsedItem.description += ` - ${propTranslator[p]??(() => {
          console.log(`Missing prop: ${p} on item ${x.name}`)
          return "Other"
        })()}\n`
      }
      parsedItem.description += '\n'
    }
    if (x.mastery) {
      parsedItem.description += `Masteries: \n`
      for (const m of x.mastery) {
        parsedItem.description += `- ${m}\n`
      }
      parsedItem.description += '\n'
    }
    if (x.range) {
      parsedItem.description += `Range: ${x.range}\n\n`
    }
    if (x.reload) {
      parsedItem.description += `Reload: ${x.reload}\n\n`
    }
    parsedItem.description += lines.join('\n\n')
    if (x.source) {
      parsedItem.description += `\n\n*From ${x.source + (x.page ? ' p.' + x.page : '')}*`
    }
    parsedItem.description = parsedItem.description.replace(regex1, (match, group1) => {
      return group1
    })
    parsedItem.description = parsedItem.description.replace(regex2, (match, group1) => {
      return group1
    })
    parsedItem.description = parsedItem.description.replace(regex3, (match, group1) => {
      return group1
    })

    parsedItemList.push(parsedItem)
  }

  // not used anymore, but should be kept to debug changes in the future
  //for (const item of parsedItemList) {
  //    await pushPresetToServer(item)
  //}
  //return

  const LOWER_BOUND_SIZE = 100 * 1000
  const UPPER_BOUND_SIZE = 200 * 1000
  while (parsedItemList.length != 0) {
    const currentTransferList: PresetList = {
      presets: []
    }
    while (
      getJsonSizeInBytes(currentTransferList) < LOWER_BOUND_SIZE &&
      parsedItemList.length != 0
    ) {
      const elementsMoving = parsedItemList.splice(0, 100)
      currentTransferList.presets.push(...elementsMoving)
    }
    if (getJsonSizeInBytes(currentTransferList) > UPPER_BOUND_SIZE) {
      const elementsMoving = currentTransferList.presets.splice(0, 100)
      parsedItemList.push(...elementsMoving)
    }
    pushPresetListToServer(currentTransferList)
  }
}

function getJsonSizeInBytes(data: unknown): number {
  const jsonString = JSON.stringify(data)
  return new TextEncoder().encode(jsonString).length
}

interface PresetList {
  presets: ItemPreset[]
}

async function pushPresetListToServer(presetList: PresetList) {
  const response = await axios
    .put<unknown>(DatabaseHandler.BASE_URL + 'itemPreset/addExtern', JSON.stringify(presetList), {
      withCredentials: true
    })
    .then((response) => response)
    .catch((error) => error.response)
  if (response && response.status >= 200 && response.status < 300) {
    return
  } else {
    ErrorHandler.getInstance().registerError(
      new Error(
        `Could put extern preset List to Server due to: ${response.status} ${response.statusText}`
      )
    )
  }
}

// not used anymore, but should be kept to debug changes in the future
// eslint-disable-next-line @typescript-eslint/no-unused-vars
async function pushPresetToServer(itemPreset: ItemPreset) {
  const plist: PresetList = {
    presets: [itemPreset]
  }
  const response = await axios
    .put<unknown>(DatabaseHandler.BASE_URL + 'itemPreset/addExtern', JSON.stringify(plist), {
      withCredentials: true
    })
    .then((response) => response)
    .catch((error) => error.response)
  if (response && response.status == 500) {
    await new Promise((resolve) => setTimeout(resolve, 1000))
    await pushPresetToServer(itemPreset)
  }
  if (response && response.status >= 200 && response.status < 300) {
    return
  } else {
    ErrorHandler.getInstance().registerError(
      new Error(
        `Could put extern itemPreset ${itemPreset.name} to Server due to: ${response.status} ${response.statusText}`
      )
    )
  }
}
