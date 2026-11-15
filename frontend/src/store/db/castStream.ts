import type { SpellPreset } from '@/model/SpellPreset'
import { accountData } from '../pinia/account'
import { BaseDatabaseHandler } from './base'
import { zauberData } from '../pinia/zauber'

export class CastStream extends BaseDatabaseHandler {
  private eventSource?: EventSource
  private castListeners: ListenerFunction[] = []

  public constructor() {
    super(BaseDatabaseHandler.getBaseURL(8001) + 'spellPreset/cast/stream/')
  }

  public async initialize(): Promise<void> {
    if (!accountData().userIsDm) return
    this.eventSource = new EventSource(this.baseURL, { withCredentials: true })
    window.addEventListener('beforeunload', () => this.eventSource?.close())
    this.eventSource.addEventListener('spellCast', (e) => {
      const data = CastStream.getData(e.data)
      for (const listener of this.castListeners) {
        listener(
          data.casterUuid,
          data.spell,
          data.castLevel,
          data.ritual,
          data.replacedConcentrationUuid
        )
      }
    })
    this.eventSource.addEventListener('concentrationEnded', (e) => {
      const owner = CastStream.getData(e.data).ownerUuid
      zauberData().concentrationSpell[owner] = undefined
    })
  }

  public addCastListener(l: ListenerFunction) {
    this.castListeners.push(l)
  }

  private static getData(data: unknown) {
    if (typeof data === 'string') return JSON.parse(data)
    return data
  }
}

type ListenerFunction = (
  account: string,
  spell: SpellPreset,
  level: number,
  ritual: boolean,
  replacedConcentrationUuid: string | null | undefined
) => void
