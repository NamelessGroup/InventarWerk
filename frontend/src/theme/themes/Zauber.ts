import type { Theme } from '../Theme'

export const zauberTheme: Theme = {
  background: 'oklch(12.9% 0.042 264.695)', //
  popUpBackground: 'oklch(20.8% 0.042 265.755)', //
  accent: 'oklch(87.9% 0.169 91.605)', //
  font: '#fff', //
  delete: 'oklch(80.8% 0.114 19.571)', //
  primary: {
    font: 'oklch(80.9% 0.105 251.813)', //
    light: 'oklch(48.8% 0.243 264.376)', //
    default: 'oklch(37.9% 0.146 265.522)', //
    dark: 'oklch(28.2% 0.091 267.935)' //
  },
  error: {
    popUpBackground: 'oklch(44.4% 0.177 26.899)', //
    popUpFont: '#fff', //
    popUpBar: 'oklch(70.4% 0.191 22.216)', //
    font: 'oklch(63.7% 0.237 25.331)' //
  }
}
