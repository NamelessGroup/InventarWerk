import type { Theme } from '../Theme'

export const inventarTheme: Theme = {
  background: 'oklch(12.9% 0.042 264.695)',
  popUpBackground: 'oklch(20.8% 0.042 265.755)',
  accent: 'oklch(87.9% 0.169 91.605)',
  font: '#fff',
  delete: 'oklch(80.8% 0.114 19.571)',
  primary: {
    font: 'oklch(83.3% 0.145 321.434)',
    light: 'oklch(49.6% 0.265 301.924)',
    default: 'oklch(40.1% 0.17 325.612)',
    dark: 'oklch(29.3% 0.136 325.661)'
  },
  error: {
    popUpBackground: 'oklch(44.4% 0.177 26.899)',
    popUpFont: '#fff',
    popUpBar: 'oklch(70.4% 0.191 22.216)',
    font: 'oklch(63.7% 0.237 25.331)'
  }
}
