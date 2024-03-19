/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum TestCaseStatus {
  Queued = 0,
  Running = 1,
  Passed = 2,
  Failed = 3,
  Cancelled = 4,
  ExpectedFailure = 5,
}
export function isAvailable(): boolean
export class BamlTracer {
  constructor()
  start(): void
  stop(): void
  flush(): void
}
export class BamlTester {
  constructor(testCases: Array<[string, string]>)
  start(): Promise<void>
  end(): Promise<void>
  updateTestCase(
    testSuite: string,
    testCase: string,
    status: TestCaseStatus,
    errorData?: any | undefined | null,
  ): Promise<void>
}
export type JsScopeGuard = BamlScopeGuard
export class BamlScopeGuard {
  static create(
    functionName: string,
    returnType: string,
    parameters: Array<[string, string]>,
    asKwarg: boolean,
  ): JsScopeGuard
  child(functionName: string, returnType: string, parameters: Array<[string, string]>, asKwarg: boolean): BamlScopeGuard
  logInputs(args: { [key: string]: any } | any[]): void
  logOutput(result?: string | undefined | null): void
  logError(errorCode: number, message?: string | undefined | null, stack?: string | undefined | null): void
  logLlmStart(event: {
    prompt:
      | string
      | {
          role: string
          content: string
        }[]
    provider: string
  }): void
  logLlmEnd(event: { model_name: string; generated: string; metadata: any }): void
  logLlmError(event: { error_code: number; message?: string; traceback?: string }): void
  logLlmCacheHit(event: number): void
  logLlmArgs(args: { [key: string]: any }): void
  logLlmTemplateArgs(args: {
    template:
      | string
      | {
          role: string
          content: string
        }[]
    template_args: {
      [key: string]: string
    }
  }): void
  logVariant(event: string): void
  setTags(event: { [key: string]: string | null }): void
  close(): void
}
