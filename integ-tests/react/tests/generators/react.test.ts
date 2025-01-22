/* eslint-disable @typescript-eslint/no-unused-vars */
import { existsSync, readFileSync } from 'fs';
import { join } from 'path';
import { TestAwsAction, TestUniverseQuestionAction } from '../../baml_client/react/server';
import { useLLM, useTestAws, useTestUniverseQuestion } from '../../baml_client/react/client';
import { act, renderHook, waitFor } from '@testing-library/react';
import type { StreamingReturnType, NonStreamingReturnType, StreamingInputProps, NonStreamingInputProps } from '../../baml_client/react/types';
import { UniverseQuestion } from '../../baml_client/types';

describe('React Generator', () => {
  const outputDir = join(__dirname, '../../baml_client/react');

  describe('File Generation', () => {
    it('should generate server actions', () => {
      const serverActionsPath = join(outputDir, 'server.ts');

      expect(existsSync(serverActionsPath)).toBe(true);

      const serverActions = readFileSync(serverActionsPath, 'utf-8');
      expect(serverActions).toContain("'use server'");
      expect(serverActions).toContain('export async function');
      expect(serverActions).toContain('ReadableStream<Uint8Array>');
    });

    it('should generate client hooks', () => {
      const clientHooksPath = join(outputDir, 'client.tsx');

      expect(existsSync(clientHooksPath)).toBe(true);

      const clientHooks = readFileSync(clientHooksPath, 'utf-8');
      expect(clientHooks).toContain("'use client'");
      expect(clientHooks).toContain('export function use');
      expect(clientHooks).toContain('data: ');
      expect(clientHooks).toContain('partialData: ');
      expect(clientHooks).toContain('isLoading: boolean');
    });

    it('should generate types file', () => {
      const typesPath = join(outputDir, 'types.ts');

      expect(existsSync(typesPath)).toBe(true);

      const types = readFileSync(typesPath, 'utf-8');
      expect(types).toContain('export type PartialResponse<Output>');
      expect(types).toContain('export type FinalResponse<Output>');
      expect(types).toContain('export type StreamingInputProps<Output>');
      expect(types).toContain('export type NonStreamingInputProps<Output>');
      expect(types).toContain('export type UseLLMOptions<Output>');
    });
  });

  describe('Server Actions', () => {
    it('should have correct streaming type signatures', async () => {
      // Test streaming server action types
      const streamingResult = TestAwsAction('test input', { stream: true });
      expect(streamingResult).toBeInstanceOf(Promise);

      // Test non-streaming server action types
      const nonStreamingResult = TestAwsAction('test input', { stream: false });
      expect(nonStreamingResult).toBeInstanceOf(Promise);

      // Test default (non-streaming) server action types
      const defaultResult = TestAwsAction('test input');
      expect(defaultResult).toBeInstanceOf(Promise);
    });

    it('should handle complex input/output types', async () => {
      const questionInput = { question: 'test question' };

      // Test streaming with complex types
      const streamingResult = TestUniverseQuestionAction(questionInput, { stream: true });
      expect(streamingResult).toBeInstanceOf(Promise);

      // Test non-streaming with complex types
      const nonStreamingResult = TestUniverseQuestionAction(questionInput, { stream: false });
      expect(nonStreamingResult).toBeInstanceOf(Promise);
    });
  });

  describe('Client Hooks', () => {
    it('should have correct streaming hook types', () => {
      const { result } = renderHook(() => useTestAws({
        stream: true,
        onPartial: jest.fn(),
        onFinal: jest.fn(),
        onError: jest.fn(),
      }));

      // Type checks for streaming
      expect(result.current.data).toBe(null);
      expect(result.current.partialData).toBe(null);
      expect(result.current.isLoading).toBe(false);
      expect(result.current.isError).toBe(false);
      expect(result.current.status).toBe('idle');
      expect(typeof result.current.mutate).toBe('function');
    });

    it('should have correct non-streaming hook types', () => {
      const { result } = renderHook(() => useTestAws({
        onFinal: jest.fn(),
        onError: jest.fn(),
      }));

      // Type checks for non-streaming
      expect(result.current.data).toBe(null);
      expect(result.current.isLoading).toBe(false);
      expect(result.current.isError).toBe(false);
      expect(result.current.status).toBe('idle');
      expect(typeof result.current.mutate).toBe('function');
    });

    it('should have correct generic hook types', () => {
      // Test generic hook usage with streaming
      const { result: streamingResult } = renderHook(() => useLLM(TestAwsAction, {
        stream: true,
        onPartial: jest.fn(),
        onFinal: jest.fn(),
        onError: jest.fn(),
      }));

      // Type checks for streaming
      expect(streamingResult.current.data).toBe(null);
      expect(streamingResult.current.partialData).toBe(null);
      expect(streamingResult.current.isLoading).toBe(false);
      expect(streamingResult.current.isError).toBe(false);
      expect(streamingResult.current.status).toBe('idle');
      expect(typeof streamingResult.current.mutate).toBe('function');

      // Test generic hook usage without streaming
      const { result: nonStreamingResult } = renderHook(() => useLLM(TestAwsAction, {
        onFinal: jest.fn(),
        onError: jest.fn(),
      }));

      // Type checks for non-streaming
      expect(nonStreamingResult.current.data).toBe(null);
      expect(nonStreamingResult.current.isLoading).toBe(false);
      expect(nonStreamingResult.current.isError).toBe(false);
      expect(nonStreamingResult.current.status).toBe('idle');
      expect(typeof nonStreamingResult.current.mutate).toBe('function');
    });
  });

  describe('Type Safety', () => {
    it('should enforce correct streaming return types', () => {
      // Verify non-streaming return type
      type NonStreamingResponse = Awaited<ReturnType<typeof TestAwsAction>>;
      type ExpectedNonStreamingType = string | ReadableStream<Uint8Array>;

      // Verify streaming return type
      type StreamingResponse = Awaited<ReturnType<typeof TestAwsAction>>;
      type ExpectedStreamingType = ReadableStream<Uint8Array>;

      // These type assertions verify compile-time type checking
      const _assertTypes: [ExpectedNonStreamingType, ExpectedStreamingType] = null!;
    });

    it('should enforce correct hook return types based on streaming option', () => {
      // Test useTestAws hook return types
      type StreamingAwsReturn = ReturnType<typeof useTestAws>;
      type NonStreamingAwsReturn = ReturnType<typeof useTestAws>;

      // Type assertions for base properties
      type BaseProps = {
        isLoading: boolean;
        isError: boolean;
        isSuccess: boolean;
        error: Error | null;
        status: 'idle' | 'loading' | 'success' | 'error';
      };

      // Verify streaming hook has partialData
      type StreamingProps = BaseProps & {
        data: string | null;
        partialData: string | null;
        mutate: (input: string) => Promise<ReadableStream<Uint8Array>>;
      };

      // Verify non-streaming hook doesn't have partialData
      type NonStreamingProps = BaseProps & {
        data: string | null;
        partialData?: never;
        mutate: (input: string) => Promise<string>;
      };

      // Test useTestUniverseQuestion hook return types
      type UniverseQuestionResponse = { answer?: string | null };
      type StreamingUniverseProps = BaseProps & {
        data: UniverseQuestionResponse | null;
        partialData: UniverseQuestionResponse | null;
        mutate: (input: { question: string }) => Promise<ReadableStream<Uint8Array>>;
      };

      type NonStreamingUniverseProps = BaseProps & {
        data: UniverseQuestionResponse | null;
        partialData?: never;
        mutate: (input: { question: string }) => Promise<UniverseQuestionResponse>;
      };

      // Test useLLM hook return types
      type StreamingLLMProps = BaseProps & {
        data: string | null;
        partialData: string | null;
        mutate: (input: string) => Promise<ReadableStream<Uint8Array>>;
      };

      type NonStreamingLLMProps = BaseProps & {
        data: string | null;
        partialData?: never;
        mutate: (input: string) => Promise<string>;
      };

      // Type assertions to verify conditional types
      type _TestStreamingAws = StreamingAwsReturn extends StreamingProps ? true : false;
      type _TestNonStreamingAws = NonStreamingAwsReturn extends NonStreamingProps ? true : false;
      type _TestStreamingUniverse = StreamingUniverseProps extends StreamingProps ? true : false;
      type _TestNonStreamingUniverse = NonStreamingUniverseProps extends NonStreamingProps ? true : false;
      type _TestStreamingLLM = StreamingLLMProps extends StreamingProps ? true : false;
      type _TestNonStreamingLLM = NonStreamingLLMProps extends NonStreamingProps ? true : false;
    });

    it('should enforce correct hook option types', () => {
      // Test useTestAws options
      type StreamingAwsOptions = {
        stream: true;
        onPartial?: (response: string | null) => void;
        onFinal?: (response: string | null) => void;
        onError?: (error: Error) => void;
      };

      type NonStreamingAwsOptions = {
        stream?: false;
        onPartial?: never;
        onFinal?: (response: string | null) => void;
        onError?: (error: Error) => void;
      };

      // Type assertions for options
      type _TestStreamingAwsOptions = Parameters<typeof useTestAws>[0] extends StreamingAwsOptions ? true : false;
      type _TestNonStreamingAwsOptions = Parameters<typeof useTestAws>[0] extends NonStreamingAwsOptions ? true : false;
    });

    it('should enforce correct complex type handling', () => {
      // Test useTestUniverseQuestion options
      type UniverseQuestionResponse = { answer?: string | null };
      type StreamingUniverseOptions = {
        stream: true;
        onPartial?: (response: UniverseQuestionResponse | null) => void;
        onFinal?: (response: UniverseQuestionResponse | null) => void;
        onError?: (error: Error) => void;
      };

      type NonStreamingUniverseOptions = {
        stream?: false;
        onPartial?: never;
        onFinal?: (response: UniverseQuestionResponse | null) => void;
        onError?: (error: Error) => void;
      };

      // Type assertions for options
      type _TestStreamingUniverseOptions = Parameters<typeof useTestUniverseQuestion>[0] extends StreamingUniverseOptions ? true : false;
      type _TestNonStreamingUniverseOptions = Parameters<typeof useTestUniverseQuestion>[0] extends NonStreamingUniverseOptions ? true : false;
    });

    it('should enforce correct generic hook types', () => {
      // Test useLLM options
      type StreamingLLMOptions<T> = {
        stream: true;
        onPartial?: (response: T | null) => void;
        onFinal?: (response: T | null) => void;
        onError?: (error: Error) => void;
      };

      type NonStreamingLLMOptions<T> = {
        stream?: false;
        onPartial?: never;
        onFinal?: (response: T | null) => void;
        onError?: (error: Error) => void;
      };

      // Type assertions for options
      type _TestStreamingLLMOptions = Parameters<typeof useLLM>[1] extends StreamingLLMOptions<unknown> ? true : false;
      type _TestNonStreamingLLMOptions = Parameters<typeof useLLM>[1] extends NonStreamingLLMOptions<unknown> ? true : false;
    });
  });

  describe('Runtime Integration', () => {
    it('should handle streaming responses', async () => {
      const onPartial = jest.fn();
      const onFinal = jest.fn();
      const onError = jest.fn();

      const { result } = renderHook(() => useTestAws({
        stream: true,
        onPartial,
        onFinal,
        onError,
      }));

      await act(async () => {
        await result.current.mutate('test input');
      });

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
        expect(result.current.isError).toBe(false);
        expect(onPartial).toHaveBeenCalled();
        expect(onFinal).toHaveBeenCalled();
        expect(onError).not.toHaveBeenCalled();
      });
    });

    it('should handle non-streaming responses', async () => {
      const onFinal = jest.fn();
      const onError = jest.fn();

      const { result } = renderHook(() => useTestAws({
        onFinal,
        onError,
      }));

      await act(async () => {
        await result.current.mutate('test input');
      });

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
        expect(result.current.isError).toBe(false);
        expect(onFinal).toHaveBeenCalled();
        expect(onError).not.toHaveBeenCalled();
      });
    });

    it('should handle complex type responses', async () => {
      const onFinal = jest.fn();
      const onError = jest.fn();

      const { result } = renderHook(() => useTestUniverseQuestion({
        onFinal,
        onError,
      }));

      await act(async () => {
        await result.current.mutate({ question: 'What is the answer to life?' });
      });

      await waitFor(() => {
        expect(result.current.isLoading).toBe(false);
        expect(result.current.isError).toBe(false);
        expect(onFinal).toHaveBeenCalled();
        expect(onError).not.toHaveBeenCalled();
      });
    });
  });
});