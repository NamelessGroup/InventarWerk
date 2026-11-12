import type { Account } from '@/model/Account'
import { accountData } from '..'
import { BaseDatabaseHandler } from './base'

export class AccountDB extends BaseDatabaseHandler {
  public constructor() {
    super(BaseDatabaseHandler.getBaseURL(8000) + 'account/')
  }

  public async initialize() {
    accountData().uuid = await this.getOwnUUID()
    accountData().accounts = await this.getAllAccounts()
    accountData().userIsDm = await this.isDM()
    accountData().isServerLocked = (await this.getServerLockStatus()) ?? false
  }

  public async isLoggedIn() {
    const response = await this.get<{ loggedIn: boolean }>(['isLoggedIn'])
    if (!response) {
      return false
    }
    return response.loggedIn
  }

  public getLogInUrl() {
    return this.baseURL + 'login'
  }

  public async logOut() {
    await this.get<undefined>(['logout'])
  }

  public async isDM(user?: string) {
    const uuid = user ?? accountData().uuid
    const result = await this.get<{ isDm: boolean }>(['isDm'], {
      account_uuid: uuid
    })
    return result?.isDm ?? false
  }

  public async getAllAccounts() {
    return (await this.get<{ accounts: Account[] }>(['get']).then((r) => r?.accounts)) ?? []
  }

  private async getOwnUUID() {
    return (await this.get<{ userUUID: string }>(['info']).then((r) => r?.userUUID)) ?? ''
  }

  public async getServerLockStatus() {
    return (await this.get<{ isLocked: boolean }>(['isLocked']))?.isLocked
  }

  public changeServerLockStatus() {
    return this.patch<unknown>(['toggleLock'])
  }
}
