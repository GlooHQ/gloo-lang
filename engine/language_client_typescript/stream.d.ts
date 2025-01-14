import { FunctionResult, FunctionResultStream, RuntimeContextManager } from './native';
export declare class BamlStream<PartialOutputType, FinalOutputType> {
    private ffiStream;
    private partialCoerce;
    private finalCoerce;
    private ctxManager;
    private task;
    private eventQueue;
    constructor(ffiStream: FunctionResultStream, partialCoerce: (result: FunctionResult) => PartialOutputType, finalCoerce: (result: FunctionResult) => FinalOutputType, ctxManager: RuntimeContextManager);
    private driveToCompletion;
    private driveToCompletionInBg;
    [Symbol.asyncIterator](): AsyncIterableIterator<PartialOutputType>;
    getFinalResponse(): Promise<FinalOutputType>;
    /**
     * Converts the BAML stream to a Next.js compatible stream.
     * This is used for server-side streaming in Next.js API routes and Server Actions.
     */
    toStreamable(): ReadableStream<Uint8Array>;
}
//# sourceMappingURL=stream.d.ts.map