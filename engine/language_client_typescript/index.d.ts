export { BamlRuntime, FunctionResult, FunctionResultStream, BamlImage as Image, ClientBuilder, BamlAudio as Audio, invoke_runtime_cli, ClientRegistry, BamlLogEvent, } from "./native";
export { BamlStream } from "./stream";
export { BamlCtxManager } from "./async_context_vars";
export declare class BamlClientFinishReasonError extends Error {
    prompt: string;
    raw_output: string;
    constructor(prompt: string, raw_output: string, message: string);
    toJSON(): string;
    static from(error: Error): BamlClientFinishReasonError | undefined;
}
export declare class BamlValidationError extends Error {
    prompt: string;
    raw_output: string;
    constructor(prompt: string, raw_output: string, message: string);
    toJSON(): string;
    static from(error: Error): BamlValidationError | undefined;
}
export declare function createBamlValidationError(error: Error): BamlValidationError | BamlClientFinishReasonError | Error;
//# sourceMappingURL=index.d.ts.map