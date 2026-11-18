import { reactive } from 'vue'

const LOCAL_STORAGE_KEY = 'invSettings'

export const SPELL_TAGS = ['level', 'components', 'concentration', 'ritual'] as const
export type SpellTag = (typeof SPELL_TAGS)[number]

export interface SettingsState {
  breakDownGold: boolean
  strictInventoryGrid: boolean
  noDeleteConfirmation: boolean
  groupSpellsByLevel: boolean
  spellTags: Record<SpellTag, boolean>
}

const DEFAULT_SETTINGS: SettingsState = {
  breakDownGold: true,
  strictInventoryGrid: false,
  noDeleteConfirmation: false,
  groupSpellsByLevel: false,
  spellTags: { level: true, components: true, concentration: true, ritual: true }
}

export class Settings {
  private static INSTANCE: Settings | null = null
  private settings: SettingsState = reactive(DEFAULT_SETTINGS)

  private constructor() {
    this.load()
  }

  public static getInstance(): Settings {
    Settings.INSTANCE ??= new Settings()
    return Settings.INSTANCE
  }

  public get breakDownGold(): boolean {
    return this.settings.breakDownGold
  }

  public get strictInventoryGrid(): boolean {
    return this.settings.strictInventoryGrid
  }

  public get noDeleteConfirmation(): boolean {
    return this.settings.noDeleteConfirmation
  }

  public get groupSpellsByLevel(): boolean {
    return this.settings.groupSpellsByLevel
  }

  public showSpellTag(tag: SpellTag): boolean {
    return this.settings.spellTags[tag]
  }

  public setShowSpellTag(tag: SpellTag, show: boolean) {
    this.settings.spellTags[tag] = show
    this.save()
  }

  public set breakDownGold(breakDownGold: boolean) {
    this.settings.breakDownGold = breakDownGold
    this.save()
  }

  public set strictInventoryGrid(strictInventoryGrid: boolean) {
    this.settings.strictInventoryGrid = strictInventoryGrid
    this.save()
  }

  public set noDeleteConfirmation(noDeleteConfirmation: boolean) {
    this.settings.noDeleteConfirmation = noDeleteConfirmation
    this.save()
  }

  public set groupSpellsByLevel(groupSpellsByLevel: boolean) {
    this.settings.groupSpellsByLevel = groupSpellsByLevel
    this.save()
  }

  private save() {
    localStorage.setItem(LOCAL_STORAGE_KEY, JSON.stringify(this.settings))
  }

  private load() {
    const settings = localStorage.getItem(LOCAL_STORAGE_KEY) ?? localStorage.getItem('settings')
    if (settings) {
      const parsedState: Partial<SettingsState> = JSON.parse(settings)
      // keep the object reactive, otherwise the UI does not update when a setting changes
      this.settings = reactive({
        ...this.settings,
        ...parsedState,
        // merged separately, so tags added later are shown by default
        spellTags: { ...this.settings.spellTags, ...parsedState.spellTags }
      })
    }
  }
}
