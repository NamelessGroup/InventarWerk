import type { Theme } from '../Theme'

export const highContrastLight: Theme = {
  background: '#fff',
  popUpBackground: '#fff',
  accent: '#000',
  font: '#000',
  delete: '#000',
  primary: {
    font: '#000',
    light: '#fff',
    default: '#fff',
    dark: '#fff'
  },
  error: {
    popUpBackground: '#fff',
    popUpFont: '#000',
    popUpBar: '#000',
    font: '#000'
  }
}

export const highContrastDark: Theme = {
  background: '#000',
  popUpBackground: '#000',
  accent: '#fff',
  font: '#fff',
  delete: '#fff',
  primary: {
    font: '#fff',
    light: '#000',
    default: '#000',
    dark: '#000'
  },
  error: {
    popUpBackground: '#000',
    popUpFont: '#fff',
    popUpBar: '#fff',
    font: '#fff'
  }
}
