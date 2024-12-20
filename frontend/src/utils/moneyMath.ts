export interface Money {
  copper: number,
  silver: number,
  gold: number,
  platinum: number
} 

export type MoneyFields = 'platinum'|'gold'|'silver'|'copper'

export function breakDownMoney(money: number): Money {
  return {
    copper: money % 10,
    silver: Math.floor(money/10) % 10,
    gold: Math.floor(money/100) % 10,
    platinum: Math.floor(money/1000)
  }
}

export function compactMoney(money: Money): number {
  return money.copper + money.silver * 10 + money.gold * 100 + money.platinum * 1000
}