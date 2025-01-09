import { b, ClientRegistry, BamlValidationError } from './test-setup'

describe('Error Handling Tests', () => {
  it('should raise an error for invalid argument types', async () => {
    await expect(async () => {
      await b.TestCaching(111 as unknown as string, 'fiction')
    }).rejects.toThrow('BamlInvalidArgumentError')
  })

  it('should raise an error for invalid client configuration', async () => {
    await expect(async () => {
      const cr = new ClientRegistry()
      cr.addLlmClient('MyClient', 'openai', { model: 'gpt-4o-mini', api_key: 'INVALID_KEY' })
      cr.setPrimary('MyClient')
      await b.MyFunc("My name is Harrison. My hair is black and I'm 6 feet tall.", { clientRegistry: cr })
    }).rejects.toThrow('BamlClientError')
  })

  it('should raise a BAMLValidationError with proper details', async () => {
    try {
      await b.DummyOutputFunction('dummy input')
      fail('Expected b.DummyOutputFunction to throw a BamlValidationError')
    } catch (error: any) {
      if (error instanceof BamlValidationError) {
        expect(error.message).toContain('BamlValidationError')
        expect(error.prompt).toContain('Say "hello there".')
        expect(error.raw_output).toBeDefined()
        expect(error.raw_output.length).toBeGreaterThan(0)
      } else {
        fail('Expected error to be an instance of BamlValidationError')
      }
    }
  })

  it('should handle client HTTP errors', async () => {
    try {
      const cr = new ClientRegistry()
      cr.addLlmClient('MyClient', 'openai', { model: 'gpt-4o-mini', api_key: 'INVALID_KEY' })
      cr.setPrimary('MyClient')
      await b.MyFunc("My name is Harrison. My hair is black and I'm 6 feet tall.", { clientRegistry: cr })
      fail('Expected b.MyFunc to throw a BamlClientHttpError')
    } catch (error: any) {
      expect(error.message).toContain('BamlClientHttpError')
    }
  })
})
