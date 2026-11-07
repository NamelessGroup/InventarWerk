import type { Theme } from './Theme'
import { highContrastDark, highContrastLight } from './themes/Contrast'
import { homeTheme } from './themes/Home'
import { inventarTheme } from './themes/Inventar'
import { zauberTheme } from './themes/Zauber'

export type Pages = 'inventar' | 'zauber' | 'home'
export type ThemeOptions = Pages | 'contrastLight' | 'contrastDark'

export const themeDictionary: Record<ThemeOptions, Theme> = {
  inventar: inventarTheme,
  zauber: zauberTheme,
  home: homeTheme,
  contrastDark: highContrastDark,
  contrastLight: highContrastLight
}
