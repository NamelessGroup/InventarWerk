import { DB } from '.'
import { BaseDatabaseHandler } from './base'

export class LastChangesStream extends BaseDatabaseHandler {
  private eventSource?: EventSource

  public constructor() {
    super(BaseDatabaseHandler.getBaseURL(8002) + 'lastChanges/stream')
  }

  public async initialize() {
    this.eventSource = new EventSource(this.baseURL, { withCredentials: true })
    window.addEventListener('beforeunload', () => this.eventSource?.close())
    this.eventSource.addEventListener('message', (e) => this.handleChange(getData(e.data)))
  }

  private handleChange(e: ChangeEvent) {
    if (e.source === 'item_preset') {
      DB.inventory.initPresets()
    } else if (e.source.startsWith('inventory') || e.source === 'notify_inventory_change_generic') {
      DB.inventory.initInventories()
    } else if (e.source === 'concentration') {
      DB.zauber.initConcentration()
    } else if (e.source.startsWith('spell_preset')) {
      DB.zauber.initPresets()
    } else if (e.source.startsWith('spell_list')) {
      DB.zauber.initSpellLists()
    } else if (e.source === 'spell_slots') {
      DB.zauber.initSpellSlots()
    } else if (e.source === 'spell_attack_modifier') {
      // not yet implemented
    } else {
      throw new Error('Unknown change source: ' + e.source)
    }
  }
}

function getData(data: unknown) {
  if (typeof data === 'string') return JSON.parse(data)
  return data
}

interface ChangeEvent {
  uuid: string
  source: string
  type: string
  timestamp: number
}

/**
concentration
inventory
inventory_item
inventory_reader
inventory_writer
item_preset
notify_inventory_change_generic
spell_attack_modifier
spell_list
spell_list_entry
spell_preset
spell_preset_class
spell_preset_component
spell_preset_feats
spell_preset_species
spell_preset_subclass
spell_slots
 */
