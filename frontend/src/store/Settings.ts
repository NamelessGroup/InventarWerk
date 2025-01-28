import { DatabaseHandler } from "./DatabaseHandler"

export class Settings {
  private static INSTACE: Settings|null = null
  private settings: SettingsState = {
    breakDownGold: true,
    timeBetweenFetches: 5
  }

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
    this.settings.timeBetweenFetches = timeBetweenFetches
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
    }
  }
}

export interface SettingsState {
  breakDownGold: boolean
  timeBetweenFetches: number
}