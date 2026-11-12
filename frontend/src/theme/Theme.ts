type ErrorDisplay = {
  popUpFont: string
  popUpBackground: string
  popUpBar: string
  font: string
}

type Primary = {
  font: string
  dark: string
  default: string
  light: string
}

export type Theme = {
  background: string
  popUpBackground: string
  font: string
  error: ErrorDisplay
  primary: Primary
  accent: string
  delete: string
}
