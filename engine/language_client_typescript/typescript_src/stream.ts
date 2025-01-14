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
      this.ffiStream.onEvent((err: Error | null, data: FunctionResult | null) => {
        if (err) {
          return
        } else {
          this.eventQueue.push(data)
        }
      })
      const retval = await this.ffiStream.done(this.ctxManager)

      return retval
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
    this.driveToCompletionInBg()

    while (true) {
      const event = this.eventQueue.shift()

      if (event === undefined) {
        await new Promise((resolve) => setTimeout(resolve, 100))
        continue
      }

      if (event === null) {
        break
      }

      if (event.isOk()) {
        yield this.partialCoerce(event.parsed())
      }
    }
  }

  async getFinalResponse(): Promise<FinalOutputType> {
    const final = await this.driveToCompletionInBg()

    return this.finalCoerce(final.parsed())
  }

  /**
   * Converts the BAML stream to a Next.js compatible stream.
   * This is used for server-side streaming in Next.js API routes and Server Actions.
   */
  toStreamable(): ReadableStream<Uint8Array> {
    const stream = this;
    return new ReadableStream({
      async start(controller) {
        try {
          for await (const partial of stream) {
            controller.enqueue(
              new TextEncoder().encode(
                JSON.stringify({ partial })
              )
            );
          }

          const final = await stream.getFinalResponse();
          controller.enqueue(
            new TextEncoder().encode(
              JSON.stringify({ final })
            )
          );
          controller.close();
        } catch (error) {
          controller.error(error);
        }
      }
    });
  }
}
