import { FunctionResult, FunctionResultStream, RuntimeContextManager } from './native'

export class BamlStream<PartialOutputType, FinalOutputType> {
  private task: Promise<FunctionResult> | null = null

  private eventQueue: (FunctionResult | null)[] = []

  constructor(
    private ffiStream: FunctionResultStream,
    private partialCoerce: (result: FunctionResult) => PartialOutputType,
    private finalCoerce: (result: FunctionResult) => FinalOutputType,
    private ctxManager: RuntimeContextManager,
  ) {}

  private async driveToCompletion(): Promise<FunctionResult> {
    try {
      this.ffiStream.onEvent((err: any, data: any) => {
        if (err) {
          console.log('errorrr', err)
          this.eventQueue.push(err)
        } else {
          console.log('data', data)
          this.eventQueue.push(data)
        }
      })
      try {
        const retval = await this.ffiStream.done(this.ctxManager)
        console.log('retval', retval)
        return retval
      } catch (err) {
        this.eventQueue.push(err)
        throw err
      }
    } finally {
      this.eventQueue.push(null)
    }
  }

  private driveToCompletionInBg(): Promise<FunctionResult> {
    if (this.task === null) {
      this.task = this.driveToCompletion()
    }

    return this.task
  }

  async *[Symbol.asyncIterator](): AsyncIterableIterator<PartialOutputType> {
    const backgroundTask = this.driveToCompletionInBg()

    while (true) {
      const event = this.eventQueue.shift()

      if (event === undefined) {
        await new Promise((resolve) => setTimeout(resolve, 100))
        continue
      }

      if (event === null) {
        break
      }

      console.log('event', event)

      if (event instanceof Error) {
        throw event
      } else if (event.code === "GenericFailure") {
        console.log('event code', event.code)
        console.log('event', event)
        console.log('event indiex 0', event[0])
        throw new Error(event[0])
      } else if (event.isOk()) {
        // event.
        yield this.partialCoerce(event.parsed())
      } else {
        throw new Error(event.error())
      }
    }
    // await backgroundTask
  }

  async getFinalResponse(): Promise<FinalOutputType> {
    const final = await this.driveToCompletionInBg()

    return this.finalCoerce(final.parsed())
  }
}
