export class ErrorHandler {
  private static INSTANCE: ErrorHandler | undefined
  private listener: Record<string, ListenerFunction>

  private constructor() {
    this.listener = {}
  }

  public static getInstance() {
    if (ErrorHandler.INSTANCE == undefined) {
      ErrorHandler.INSTANCE = new ErrorHandler()
    }
    return ErrorHandler.INSTANCE
  }

  public addListener(f: ListenerFunction) {
    const uuid = Math.random().toFixed(36).substring(7)
    this.listener[uuid] = f
    return uuid
  }

  public removeListener(uuid: string) {
    delete this.listener[uuid]
  }

  public registerError(e: Error) {
    const uuids = Object.keys(this.listener)
    for (const u of uuids) {
      this.listener[u](e)
    }
  }
}

type ListenerFunction = (e: Error) => void
