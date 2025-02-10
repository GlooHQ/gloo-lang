import { BamlClientFinishReasonError } from './index'
import { FunctionResult, FunctionResultStream, RuntimeContextManager } from './native'

export class BamlStream<PartialOutputType, FinalOutputType> {
  private task: Promise<FunctionResult> | null = null
  private eventQueue: (FunctionResult | null)[] = []
  private isCancelled = false

  constructor(
    private ffiStream: FunctionResultStream,
    private partialCoerce: (result: FunctionResult) => PartialOutputType,
    private finalCoerce: (result: FunctionResult) => FinalOutputType,
    private ctxManager: RuntimeContextManager,
  ) {}

  /**
   * Cancels the stream and stops processing
   */
  cancel(): void {
    if (this.isCancelled) {
      console.log('[TS] Stream already cancelled')
      return
    }

    console.log('[TS] Cancelling stream...')
    this.isCancelled = true
    this.ctxManager.setCancelled()
    console.log('[TS] Context cancelled')
    // Clear the queue to stop iteration
    this.eventQueue = []
    this.eventQueue.push(null)
    console.log('[TS] Event queue cleared')
  }

  private async driveToCompletion(): Promise<FunctionResult> {
    try {
      console.log('[TS] Setting up event handler')
      this.ffiStream.onEvent((err: unknown, data: FunctionResult) => {
        if (err) {
          console.log('[TS] Event error:', err)
          return
        } else {
          console.log('[TS] Received event data')
          this.eventQueue.push(data)
        }
      })
      console.log('[TS] Waiting for stream completion')
      const retval = await this.ffiStream.done(this.ctxManager)
      console.log('[TS] Stream completed')

      return retval
    } finally {
      console.log('[TS] Pushing final null to queue')
      this.eventQueue.push(null)
    }
  }

  private driveToCompletionInBg(): Promise<FunctionResult> {
    if (this.task === null) {
      console.log('[TS] Starting background task')
      this.task = this.driveToCompletion()
    }

    return this.task
  }

  async *[Symbol.asyncIterator](): AsyncIterableIterator<PartialOutputType> {
    this.driveToCompletionInBg()

    while (!this.isCancelled) {
      const event = this.eventQueue.shift()

      if (event === undefined) {
        await new Promise((resolve) => setTimeout(resolve, 100))
        continue
      }

      if (event === null) {
        console.log('[TS] Iterator received null event, breaking')
        break
      }

      if (event.isOk()) {
        console.log('[TS] Yielding event data')
        yield this.partialCoerce(event.parsed(true))
      }
    }

    if (this.isCancelled) {
      console.log('[TS] Iterator throwing cancellation error')
      throw new BamlClientFinishReasonError('', '', 'Stream was cancelled', 'Stream was cancelled')
    }
  }

  async getFinalResponse(): Promise<FinalOutputType> {
    if (this.isCancelled) {
      console.log('[TS] getFinalResponse throwing cancellation error')
      throw new BamlClientFinishReasonError('', '', 'Stream was cancelled', 'Stream was cancelled')
    }

    console.log('[TS] Getting final response')
    const final = await this.driveToCompletionInBg()
    return this.finalCoerce(final.parsed(false))
  }
}
