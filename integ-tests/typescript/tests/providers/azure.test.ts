import { b } from '../test-setup'

describe('Azure Provider', () => {
  it('should support azure', async () => {
    const res = await b.TestAzure('Donkey Kong')
    expect(res.toLowerCase()).toContain('donkey')
  })

  it('should fail if azure is not configured', async () => {
    await expect(async () => {
      await b.TestAzureFailure('Donkey Kong')
    }).rejects.toThrow('BamlClientError')
  })
})
