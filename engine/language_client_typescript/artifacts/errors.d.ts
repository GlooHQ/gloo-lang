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
//# sourceMappingURL=errors.d.ts.map