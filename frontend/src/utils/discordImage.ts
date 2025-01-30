import type { Account } from "@/model/Account";

export function discordImage (account: Account): string {
  if (account.avatar === null || account.avatar === undefined || account.avatar === "" || account.avatar === "null") {
    return `https://cdn.discordapp.com/embed/avatars/${parseInt(account.uuid) % 6}.png`
  }
  return `https://cdn.discordapp.com/avatars/${account.uuid}/${account.avatar}.png`
}