import { b } from '../test-setup'

describe('Vertex Provider', () => {
  it('should support vertex', async () => {
    const res = await b.TestVertex('Donkey Kong')
    expect(res.toLowerCase()).toContain('donkey')
  })
})
