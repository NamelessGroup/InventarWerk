import { DatabaseHandler } from "./DatabaseHandler"

export interface SettingsState {
  breakDownGold: boolean
  timeBetweenFetches: number
}

const DEFAULT_SETTINGS: SettingsState = {
  breakDownGold: true,
  timeBetweenFetches: 5
}

type SettingsKey = keyof SettingsState

export class Settings {
  private static INSTACE: Settings|null = null
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

  public set breakDownGold(breakDownGold: boolean) {
    this.settings.breakDownGold = breakDownGold
    this.save()
  }

  public set timeBetweenFetches(timeBetweenFetches: number) {
    this.settings.timeBetweenFetches = timeBetweenFetches ?? 5
    this.save()
    DatabaseHandler.getInstance().setFetchInterval(timeBetweenFetches)
  }

  private save() {
    localStorage.setItem('settings', JSON.stringify(this.settings))
  }

  private load() {
    const settings = localStorage.getItem('settings')
    if (settings) {
      this.settings = JSON.parse(settings)
      const keys = Object.keys(DEFAULT_SETTINGS) as SettingsKey[]
      for (const key of keys) {
        if (this.settings[key] === undefined) {
          // @ts-ignore
          this.settings[key] = DEFAULT_SETTINGS[key]
        }
      }
    }
  }
}