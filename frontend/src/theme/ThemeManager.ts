import { computed, ref } from 'vue'
import { themeDictionary, type Pages, type ThemeOptions } from './ThemeDicitionary'

const LOCAL_STORAGE_KEY = 'invTheme'

export class ThemeManager {
  private static INSTANCE: ThemeManager
  private _pageTheme = ref<Pages>('inventar')
  private _overwriteTheme = ref<ThemeOptions | undefined>(undefined)
  private _appliedTheme = computed<ThemeOptions>(() =>
    this._overwriteTheme.value !== undefined ? this._overwriteTheme.value : this._pageTheme.value
  )

  private constructor() {
    const localTheme = localStorage.getItem(LOCAL_STORAGE_KEY)
    if (localTheme === 'undefined' || localTheme === null) {
      this._overwriteTheme.value = undefined
    } else {
      this._overwriteTheme.value = localTheme as ThemeOptions
    }
  }

  public static getInstance(): ThemeManager {
    if (!this.INSTANCE) {
      this.INSTANCE = new ThemeManager()
    }
    return this.INSTANCE
  }

  set pageTheme(page: Pages) {
    this._pageTheme.value = page
    this.updateTheme()
  }

  set overwriteTheme(t: ThemeOptions | undefined) {
    this._overwriteTheme.value = t
    localStorage.setItem(LOCAL_STORAGE_KEY, this._overwriteTheme.value ?? 'undefined')
    this.updateTheme()
  }

  get overwriteTheme() {
    return this._overwriteTheme.value
  }

  get appliedTheme() {
    return this._appliedTheme.value
  }

  private updateTheme() {
    const usedTheme = themeDictionary[this._appliedTheme.value]
    const variables = buildKVPairs(usedTheme, '--col')
    const root = document.documentElement
    for (const key of Object.keys(variables)) {
      root.style.setProperty(key, variables[key])
    }
  }
}

interface O {
  [key: string]: string | O
}

function buildKVPairs(o: O, prefix: string): Record<string, string> {
  const r: Record<string, string> = {}
  for (const k of Object.keys(o)) {
    const completeKey = prefix + capatalize(k)
    if (typeof o[k] === 'string') {
      r[completeKey] = o[k]
    } else {
      const children = buildKVPairs(o[k], completeKey)
      for (const cK of Object.keys(children)) {
        r[cK] = children[cK]
      }
    }
  }
  return r
}

function capatalize(s: string) {
  return s.substring(0, 1).toUpperCase() + s.substring(1)
}
