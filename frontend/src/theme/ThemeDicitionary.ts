import type { Theme } from './Theme'
import { highContrastDark, highContrastLight } from './themes/Contrast'
import { InventarTheme } from './themes/Inventar'

export type Pages = 'inventar'
export type ThemeOptions = Pages | 'contrastLight' | 'contrastDark'

export const themeDictionary: Record<ThemeOptions, Theme> = {
  inventar: InventarTheme,
  contrastDark: highContrastDark,
  contrastLight: highContrastLight
}
