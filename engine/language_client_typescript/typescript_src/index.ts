export {
  BamlRuntime,
  FunctionResult,
  FunctionResultStream,
  BamlImage as Image,
  ClientBuilder,
  BamlAudio as Audio,
  invoke_runtime_cli,
  ClientRegistry,
  BamlLogEvent,
} from "./native";
export { BamlStream } from "./stream";
export { BamlCtxManager } from "./async_context_vars";

export class BamlClientFinishReasonError extends Error {
  prompt: string;
  raw_output: string;

  constructor(prompt: string, raw_output: string, message: string) {
    super(message);
    this.name = "BamlClientFinishReasonError";
    this.prompt = prompt;
    this.raw_output = raw_output;

    Object.setPrototypeOf(this, BamlClientFinishReasonError.prototype);
  }

  toJSON(): string {
    return JSON.stringify(
      {
        name: this.name,
        message: this.message,
        raw_output: this.raw_output,
        prompt: this.prompt,
      },
      null,
      2
    );
  }

  static from(error: Error): BamlClientFinishReasonError | undefined {
    if (error.message.includes("BamlClientFinishReasonError")) {
      try {
        const errorData = JSON.parse(error.message);
        if (errorData.type === "BamlClientFinishReasonError") {
          return new BamlClientFinishReasonError(
            errorData.prompt || "",
            errorData.raw_output || "",
            errorData.message || error.message
          );
        } else {
          console.warn("Not a BamlClientFinishReasonError:", error);
        }
      } catch (parseError) {
        // If JSON parsing fails, fall back to the original error
        console.warn("Failed to parse BamlClientFinishReasonError:", parseError);
      }
    }
    return undefined;
  }
}

export class BamlValidationError extends Error {
  prompt: string;
  raw_output: string;

  constructor(prompt: string, raw_output: string, message: string) {
    super(message);
    this.name = "BamlValidationError";
    this.prompt = prompt;
    this.raw_output = raw_output;

    Object.setPrototypeOf(this, BamlValidationError.prototype);
  }

  toJSON(): string {
    return JSON.stringify(
      {
        name: this.name,
        message: this.message,
        raw_output: this.raw_output,
        prompt: this.prompt,
      },
      null,
      2
    );
  }

  static from(error: Error): BamlValidationError | undefined {
    if (error.message.includes("BamlValidationError")) {
      try {
        const errorData = JSON.parse(error.message);
        if (errorData.type === "BamlValidationError") {
          return new BamlValidationError(
            errorData.prompt || "",
            errorData.raw_output || "",
            errorData.message || error.message
          );
        }
      } catch (parseError) {
        console.warn("Failed to parse BamlValidationError:", parseError);
      }
    }
    return undefined;
  }
}

// Helper function to safely create a BamlValidationError
export function createBamlValidationError(
  error: Error
): BamlValidationError | BamlClientFinishReasonError | Error {
  const bamlValidationError = BamlValidationError.from(error);
  if (bamlValidationError) {
    return bamlValidationError;
  }

  const bamlClientFinishReasonError = BamlClientFinishReasonError.from(error);
  if (bamlClientFinishReasonError) {
    return bamlClientFinishReasonError;
  }

  // otherwise return the original error
  return error;
}

// No need for a separate throwBamlValidationError function in TypeScript
