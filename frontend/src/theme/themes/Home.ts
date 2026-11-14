import type { Theme } from '../Theme'

export const homeTheme: Theme = {
  background: 'oklch(15.3% 0.006 107.1)',
  popUpBackground: 'oklch(22.8% 0.013 107.4)',
  accent: 'oklch(87.9% 0.169 91.605)',
  font: '#fff',
  delete: 'oklch(80.8% 0.114 19.571)',
  primary: {
    font: 'oklch(84.5% 0.143 164.978)',
    light: 'oklch(50.8% 0.118 165.612)',
    default: 'oklch(37.8% 0.077 168.94)',
    dark: 'oklch(26.2% 0.051 172.552)'
  },
  error: {
    popUpBackground: 'oklch(44.4% 0.177 26.899)',
    popUpFont: '#fff',
    popUpBar: 'oklch(70.4% 0.191 22.216)',
    font: 'oklch(63.7% 0.237 25.331)'
  }
}
