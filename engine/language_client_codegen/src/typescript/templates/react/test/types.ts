import type { BamlStream } from '@boundaryml/baml';
import type { RecursivePartialNull } from '../types';

/**
 * Type for representing a partial response with type safety
 * @template Output The type of the partial response data
 */
export type PartialResponse<Output> = {
  partial?: RecursivePartialNull<Output> | null
  final?: never
}

/**
 * Type for representing a final response with type safety
 * @template Output The type of the final response data
 */
export type FinalResponse<Output> = {
  partial?: never
  final: Output
}

/**
 * Configuration for streaming mode, which provides incremental updates.
 * Use this when you want to show partial results as they become available.
 *
 * @template Output Type of the incremental response chunks
 */
export type StreamingInputProps<Output> = {
  stream: true
  onPartial?: (response: RecursivePartialNull<Output>) => void
  onFinal?: (response: Output) => void
}

/**
 * Options interface for non-streaming mode.
 * @template Output The type of the final response data
 */
export type NonStreamingInputProps<Output> = {
  stream?: false
  onPartial?: never
  onFinal?: (response: Output) => void
}

/**
 * Union type of all possible options.
 * @template Output The type of the response data
 */
export type UseLLMOptions<Output> = (StreamingInputProps<Output> | NonStreamingInputProps<Output>) & {
  /** Called if the operation fails */
  onError?: (error: Error) => void
}

/**
 * Type definition for a streaming server action.
 * @template Input The type of input parameters
 * @template Output The type of the response data
 */
export type ServerActionType<Output, Input extends unknown[]> = (...input: Input) => Output

/**
 * Type guard to check if options are for streaming mode
 * @template Output The type of the response data
 */
export function isStreamingOptions<Output>(
  options: UseLLMOptions<Output>
): options is StreamingInputProps<Output> {
  return options.stream === true;
}

/**
 * The complete state and controls for a BAML operation.
 * Contains everything needed to track progress and handle results.
 */
export interface BaseReturnType<Output> {
  /**
   * The complete, final result of the operation.
   * Only available after successful completion (when isSuccess is true).
   * Null during loading or if an error occurred.
   */
  data: Output | null;
  /**
   * Error details if the operation failed.
   * Check this when isError is true to handle the failure.
   */
  error: Error | null;
  /**
   * True if the operation failed.
   * Use this to conditionally render error states or retry options.
   */
  isError: boolean;
  /**
   * True while the operation is in progress.
   * Use this to show loading states, spinners, or disable controls.
   */
  isLoading: boolean;
  /**
   * True if the operation completed successfully.
   * Check this before accessing the final data.
   */
  isSuccess: boolean;
  /**
   * The current phase of the operation:
   * - idle: Initial state, ready to start
   * - loading: Operation in progress
   * - success: Completed successfully
   * - error: Failed with an error
   */
  status: "idle" | "loading" | "success" | "error";
}

/**
 * Additional state available in streaming mode.
 * Use this to handle incremental updates during processing.
 */
export type StreamingReturnType<Output, Input extends unknown[]> =
  BaseReturnType<Output> & {
    /**
     * The most recent partial result from the stream.
     * Updates continuously while streaming, showing interim progress.
     * Use this to implement real-time UI updates, typing effects,
     * or progress indicators.
     */
    partialData: RecursivePartialNull<Output>;

     /**
   * Call this function to start the operation.
   * Returns a promise that resolves with the final result or null if it failed.
   */
   mutate: (...args: Input) => Promise<ReadableStream<Uint8Array>>;
}

/**
 * Return type for non-streaming mode, extends base return type.
 * @template Output The type of the final response data
 * @template Input Tuple type of function parameters
 */
export type NonStreamingReturnType<Output, Input extends unknown[]> =
  BaseReturnType<Output> & {
    /** Not available in non-streaming mode */
    partialData: never;

     /**
   * Call this function to start the operation.
   * Returns a promise that resolves with the final result or null if it failed.
   */
     mutate: (...args: Input) => Promise<Output>;
    }
/**
 * Conditional return type based on the options provided.
 * Returns StreamingReturnType if streaming is enabled, otherwise NonStreamingReturnType.
 */
export type UseLLMReturnType<Output, Input extends unknown[], TOptions extends UseLLMOptions<Output>> =
  TOptions extends { stream: true }
    ? StreamingReturnType<Output, Input>
    : NonStreamingReturnType<Output, Input>;


