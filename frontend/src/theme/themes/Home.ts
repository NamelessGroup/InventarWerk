import type { Theme } from '../Theme'

export const homeTheme: Theme = {
  background: 'oklch(14.5% 0.008 326)',
  popUpBackground: 'oklch(21.2% 0.019 322.12)',
  accent: 'oklch(87.9% 0.169 91.605)',
  font: '#fff',
  delete: 'oklch(80.8% 0.114 19.571)',
  primary: {
    font: 'oklch(80.8% 0.114 19.571)',
    light: 'oklch(50.5% 0.213 27.518)',
    default: 'oklch(39.6% 0.141 25.723)',
    dark: 'oklch(25.8% 0.092 26.042)'
  },
  error: {
    popUpBackground: 'oklch(44.4% 0.177 26.899)',
    popUpFont: '#fff',
    popUpBar: 'oklch(70.4% 0.191 22.216)',
    font: 'oklch(63.7% 0.237 25.331)'
  }
}
