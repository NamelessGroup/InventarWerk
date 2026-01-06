import { DatabaseHandler } from './DatabaseHandler'

export interface SettingsState {
  breakDownGold: boolean
  timeBetweenFetches: number
  strictInventoryGrid: boolean
}

const DEFAULT_SETTINGS: SettingsState = {
  breakDownGold: true,
  timeBetweenFetches: 5,
  strictInventoryGrid: false
}

export class Settings {
  private static INSTACE: Settings | null = null
  private settings: SettingsState = DEFAULT_SETTINGS

  private constructor() {
    this.load()
  }

  public static getInstance(): Settings {
    if (Settings.INSTACE === null) {
      Settings.INSTACE = new Settings()
    }
    return Settings.INSTACE
  }

  public get breakDownGold(): boolean {
    return this.settings.breakDownGold
  }

  public get timeBetweenFetches(): number {
    return this.settings.timeBetweenFetches
  }

  public get strictInventoryGrid(): boolean {
    return this.settings.strictInventoryGrid
  }

  public set breakDownGold(breakDownGold: boolean) {
    this.settings.breakDownGold = breakDownGold
    this.save()
  }

  public set timeBetweenFetches(timeBetweenFetches: number) {
    this.settings.timeBetweenFetches = timeBetweenFetches ?? 5
    this.save()
    DatabaseHandler.getInstance().setFetchInterval(timeBetweenFetches)
  }

  public set strictInventoryGrid(strictInventoryGrid: boolean) {
    this.settings.strictInventoryGrid = strictInventoryGrid
    this.save()
  }

  private save() {
    localStorage.setItem('settings', JSON.stringify(this.settings))
  }

  private load() {
    const settings = localStorage.getItem('settings')
    if (settings) {
      const parsedState: Partial<Settings> = JSON.parse(settings)
      this.settings = {
        ...this.settings,
        ...parsedState
      }
    }
  }
}
