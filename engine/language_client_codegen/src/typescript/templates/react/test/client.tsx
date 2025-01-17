'use client'

import { useCallback, useMemo, useReducer } from 'react';
import type {
  ServerActionType,
  PartialResponse,
  FinalResponse,
  UseLLMOptions,
  UseLLMReturnType,
  StreamingInputProps,
  NonStreamingInputProps,
} from './types';
import { RecursivePartialNull } from '../types';
import { Image, Audio } from "@boundaryml/baml"
import * as ServerActions from './server';

// Type guard functions
function isPartialResponse<T>(obj: any): obj is PartialResponse<T> {
  return obj && 'partial' in obj && !('final' in obj);
}

function isFinalResponse<T>(obj: any): obj is FinalResponse<T> {
  return obj && 'final' in obj && !('partial' in obj);
}

/**
 * Type guard to check if options are for streaming mode
 * @template Output The type of the response data
 */
export function isStreamingOptions<Output>(
  options: UseLLMOptions<Output>
): options is StreamingInputProps<Output> {
  return options.stream === true;
}


interface LLMState<TPartial, TFinal> {
  isLoading: boolean;
  isSuccess: boolean;
  error: Error | null;
  data: TFinal | null;
  partialData: TPartial | null;
}

type LLMAction<TPartial, TFinal> =
  | { type: 'START_REQUEST' }
  | { type: 'SET_ERROR'; payload: Error }
  | { type: 'SET_PARTIAL'; payload: TPartial }
  | { type: 'SET_FINAL'; payload: TFinal }
  | { type: 'RESET' };

function llmReducer<TPartial, TFinal>(
  state: LLMState<TPartial, TFinal>,
  action: LLMAction<TPartial, TFinal>
): LLMState<TPartial, TFinal> {
  switch (action.type) {
    case 'START_REQUEST':
      return {
        isLoading: true,
        isSuccess: false,
        error: null,
        data: null,
        partialData: null,
      };
    case 'SET_ERROR':
      return {
        ...state,
        isLoading: false,
        error: action.payload,
      };
    case 'SET_PARTIAL':
      return {
        ...state,
        partialData: action.payload,
      };
    case 'SET_FINAL':
      return {
        ...state,
        isLoading: false,
        isSuccess: true,
        data: action.payload,
      };
    case 'RESET':
      return {
        isLoading: false,
        isSuccess: false,
        error: null,
        data: null,
        partialData: null,
      };
    default:
      return state;
  }
}

/**
 * A React hook for making BAML function calls with support for both streaming and non-streaming modes.
 * Provides a unified interface for handling loading states, errors, and data updates.
 *
 * @template TPartial The type of partial/intermediate response data
 * @template TFinal The type of the final response data
 * @template TParams Tuple type of function parameters
 *
 * @param serverAction The server action function to execute
 * @param options Configuration options for the hook
 *
 * @returns An object containing the current state of the operation and a mutate function to trigger it
 *
 * @example
 * ```tsx
 * // Non-streaming usage
 * const {
 *   data,           // The final result (TFinal | null)
 *   isLoading,      // Whether the request is in progress
 *   isSuccess,      // Whether the request completed successfully
 *   error,          // Any error that occurred
 *   mutate         // Function to trigger the request
 * } = useLLM(extractResume, {
 *   onFinal: (response) => console.log('Final:', response.final),
 * });
 *
 * // Streaming usage
 * const {
 *   data,           // The final result (TFinal | null)
 *   partialData,    // The latest partial result (TPartial | null)
 *   isLoading,      // Whether the request is in progress
 *   isSuccess,      // Whether the request completed successfully
 *   error,          // Any error that occurred
 *   mutate         // Function to trigger the request
 * } = useLLM(extractResume, {
 *   stream: true,
 *   onPartial: (response) => console.log('Partial:', response.partial),
 *   onFinal: (response) => console.log('Final:', response.final),
 * });
 *
 * // Trigger the request
 * await mutate({ text: "Some text to process" });
 * ```
 */
export function useLLM<Output, Input extends unknown[]>(
  action: ServerActionType<Output, Input>,
  options?: StreamingInputProps<Output>
): UseLLMReturnType<Output, Input, StreamingInputProps<Output>>;

export function useLLM<Output, Input extends unknown[]>(
  action: ServerActionType<Output, Input>,
  options?: NonStreamingInputProps<Output>
): UseLLMReturnType<Output, Input, NonStreamingInputProps<Output>>;

// Implementation
export function useLLM<Output, Input extends unknown[]>(
  serverAction: ServerActionType<Output, Input>,
  options: UseLLMOptions<Output> = {}
): UseLLMReturnType<Output, Input, typeof options> {
  const { onFinal, onError, onPartial } = options;
  const isStreaming = isStreamingOptions(options);

  const [state, dispatch] = useReducer(llmReducer<RecursivePartialNull<Output>, Output>, {
    isLoading: false,
    isSuccess: false,
    error: null,
    data: null,
    partialData: null,
  });

  const mutate = useCallback(
    async (...params: Input) => {
      dispatch({ type: 'START_REQUEST' });

      try {
        const response = await serverAction(...params);

        if (isStreaming && response instanceof ReadableStream) {
          const reader = response.getReader();
          const decoder = new TextDecoder();

          try {
            while (true) {
              const { value, done } = await reader.read();

              if (done) break;

              if (value) {
                const chunk = decoder.decode(value, { stream: true }).trim();
                try {
                  const parsed = JSON.parse(chunk);
                  if (isPartialResponse<Output>(parsed) && parsed.partial !== null) {
                    const partialValue = parsed.partial;
                    dispatch({ type: 'SET_PARTIAL', payload: partialValue });
                    onPartial?.(partialValue);
                  } else if (isFinalResponse<Output>(parsed)) {
                    const finalValue = parsed.final;
                    dispatch({ type: 'SET_FINAL', payload: finalValue });
                    onFinal?.(finalValue);
                    return finalValue;
                  }
                } catch (err) {
                  // If JSON parsing fails, treat the chunk as a raw string partial update
                  dispatch({ type: 'SET_PARTIAL', payload: chunk});
                  onPartial?.(chunk);
                }
              }
            }
          } catch (err) {
            throw err instanceof Error ? err : new Error(String(err));
          } finally {
            reader.releaseLock();
          }
          return response;
        }

        // Non-streaming case
        dispatch({ type: 'SET_FINAL', payload: response });
        onFinal?.(response);
        return response;
      } catch (error_) {
        const error = error_ instanceof Error ? error_ : new Error(String(error_));
        dispatch({ type: 'SET_ERROR', payload: error });
        onError?.(error);
        return null;
      }
    },
    [serverAction, isStreaming, onPartial, onFinal, onError],
  );

  const status = useMemo<"idle" | "loading" | "success" | "error">(() => {
    if (state.isLoading) return "loading";
    if (state.error) return "error";
    if (state.isSuccess) return "success";
    return "idle";
  }, [state.isLoading, state.error, state.isSuccess]);

  const result = {
    data: state.data,
    error: state.error,
    isError: state.error !== null,
    isLoading: state.isLoading,
    isSuccess: state.isSuccess,
    mutate,
    status,
  };

  return {
    ...result,
    partialData: isStreaming ? state.partialData : undefined,
  }
}

const Chat: ServerActionType<string, [string]> = (input: string) => {
  return 'test' as string;
}

export function useChat(
  options?: StreamingInputProps<string>
): UseLLMReturnType<string, [string], StreamingInputProps<string>>;

export function useChat(
  options?: NonStreamingInputProps<string>
): UseLLMReturnType<string, [string], NonStreamingInputProps<string>>;

// Implementation
export function useChat(
  options: UseLLMOptions<string> = {},
): UseLLMReturnType<string, [string], typeof options> {
  // NOTE: This is a hack to get the type inference to work.
  if (isStreamingOptions(options)) {
    return useLLM<string, [string]>(Chat, options);
  }

  return useLLM<string, [string]>(Chat, options);
}

/**
 * A specialized hook for the {{ func.name }} BAML function that handles both streaming and non-streaming responses.
 *
 * Input Types:
 * {%- for (name, optional, type) in func.args %}
 * - {{ name }}{% if optional %} (optional){% endif %}: {{ type }}
 * {%- endfor %}
 *
 * Return Type:
 * - Non-streaming: {{ func.return_type }}
 * - Streaming Partial: RecursivePartialNull<{{ func.return_type }}>
 * - Streaming Final: {{ func.return_type }}
 *
 * Common Usage Patterns:
 * 1. Non-streaming (Default)
 *    - Best for: Quick responses, simple UI updates
 *    - Avoid when: Response takes >5s or UI needs progressive updates
 *
 * 2. Streaming
 *    - Best for: Long-running operations, real-time UI feedback
 *    - Required when: Using features like chat interfaces or progress indicators
 *
 * Edge Cases & Gotchas:
 * 1. Error Handling
 *    - Network failures won't trigger onPartial/onFinal
 *    - Always implement onError for graceful degradation
 *    - Check error.message for specific failure reasons
 *
 * 2. Streaming Mode
 *    - partialData may be null even after updates (handle this case!)
 *    - Stream can end without final data (connection loss)
 *    - Partial results may be incomplete/invalid
 *
 * 3. State Management
 *    - data persists after completion (clear if needed)
 *    - isLoading stays true until final/error
 *    - Multiple rapid calls can race (latest wins)
 *
 * @param props Configuration options
 * @returns Hook state and controls
 *
 * @example
 * ```tsx
 * // 1. Basic Usage (Non-streaming)
 * const { data, error, isLoading, mutate } = use{{ func.name }}();
 *
 * // Handle the response
 * useEffect(() => {
 *   if (data) {
 *     // Type-safe access to {{ func.return_type }}
 *     console.log('Success:', data);
 *   }
 * }, [data]);
 *
 * // 2. Streaming with Progress
 * const {
 *   data,        // Type: {{ func.return_type }} | null
 *   partialData, // Type: RecursivePartialNull<{{ func.return_type }}> | null
 *   isLoading,
 *   error,
 *   mutate
 * } = use{{ func.name }}({
 *   stream: true,
 *
 *   // Handle partial updates (may be incomplete!)
 *   onPartial: (response) => {
 *     // Type: PartialResponse<RecursivePartialNull<{{ func.return_type }}>>
 *     console.log('Partial:', response.partial);
 *   },
 *
 *   // Handle successful completion
 *   onFinal: (response) => {
 *     // Type: FinalResponse<{{ func.return_type }}>
 *     console.log('Final:', response.final);
 *   },
 *
 *   // Robust error handling
 *   onError: (error) => {
 *     if (error.message.includes('network')) {
 *       // Handle connection issues
 *     } else if (error.message.includes('timeout')) {
 *       // Handle timeouts
 *     } else {
 *       // Handle other errors
 *     }
 *   }
 * });
 *
 * // 3. Making the Request
 * const handleSubmit = async () => {
 *   try {
 *     const result = await mutate({
 *       // Type-safe parameters:
{%- for (name, optional, type) in func.args %}
 *       {{ name }}: someValue as {{ type }},  // Replace someValue with your data
{%- endfor %}
 *     });
 *
 *     if (result) {
 *       // Success case
 *     }
 *   } catch (e) {
 *     // Handle any synchronous errors
 *   }
 * };
 *
 * // 4. Race Condition Handling
 * const handleMultipleCalls = async () => {
 *   // Only the latest call's results will be reflected in the UI
 *   const results = await Promise.all([
 *     mutate({
{%- for (name, optional, type) in func.args %}
 *       {{ name }}: firstValue as {{ type }},
{%- endfor %}
 *     }),
 *     mutate({
{%- for (name, optional, type) in func.args %}
 *       {{ name }}: secondValue as {{ type }},
{%- endfor %}
 *     })
 *   ]);
 *   // Check results[1] for the final state
 * };
 * ```
 */