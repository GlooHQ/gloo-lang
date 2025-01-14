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
<<<<<<< HEAD:engine/language_client_typescript/index.d.ts
export declare class BamlClientHttpError extends Error {
    client_name: string;
    status_code: number;
    constructor(client_name: string, message: string, status_code: number);
    toJSON(): string;
    static from(error: Error): BamlClientHttpError | undefined;
}
export declare function toBamlError(error: any): any;
//# sourceMappingURL=index.d.ts.map
=======
export declare function createBamlValidationError(error: Error): BamlValidationError | BamlClientFinishReasonError | Error;
//# sourceMappingURL=errors.d.ts.map
>>>>>>> 76e0a2db (feat: generation for react working):engine/language_client_typescript/artifacts/errors.d.ts
