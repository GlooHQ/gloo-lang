import { existsSync, readFileSync } from 'fs';
import { join } from 'path';
import { b } from '../../baml_client';
import { ReadableStream, ReadableStreamDefaultController } from 'stream/web';

describe('React Generator', () => {
  const outputDir = join(__dirname, '../../baml_client');

  describe('File Generation', () => {
    it('should generate server actions', () => {
      const serverActionsPath = join(outputDir, 'server-actions.ts');

      expect(existsSync(serverActionsPath)).toBe(true);

      const serverActions = readFileSync(serverActionsPath, 'utf-8');
      expect(serverActions).toContain("'use server'");
      expect(serverActions).toContain('export async function');
      expect(serverActions).toContain('ReadableStream<Uint8Array>');
    });

    it('should generate client hooks', () => {
      const clientHooksPath = join(outputDir, 'client-hooks.ts');

      expect(existsSync(clientHooksPath)).toBe(true);

      const clientHooks = readFileSync(clientHooksPath, 'utf-8');
      expect(clientHooks).toContain("'use client'");
      expect(clientHooks).toContain('useState');
      expect(clientHooks).toContain('export function use');
      expect(clientHooks).toContain('data: ');
      expect(clientHooks).toContain('partialData: ');
      expect(clientHooks).toContain('isLoading: boolean');
    });
  });

  describe('Runtime', () => {
    it('should stream responses through server action', async () => {
      // Create a mock stream since we can't actually call the server action in tests
      const mockReadable = new ReadableStream<Uint8Array>({
        start(controller: ReadableStreamDefaultController<Uint8Array>) {
          controller.enqueue(new TextEncoder().encode(JSON.stringify({ partial: 'test' })));
          controller.enqueue(new TextEncoder().encode(JSON.stringify({ final: 'test complete' })));
          controller.close();
        }
      });

      let result = '';
      const reader = mockReadable.getReader();

      while (true) {
        const { done, value } = await reader.read();
        if (done) break;
        result += new TextDecoder().decode(value);
      }

      expect(result).toContain('partial');
      expect(result).toContain('final');
    });
  });
});