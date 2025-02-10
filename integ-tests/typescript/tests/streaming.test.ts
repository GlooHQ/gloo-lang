import { b } from './test-setup'

describe('Streaming Tests', () => {
  it('should properly cancel a stream', async () => {
    // Start a streaming operation
    const stream = b.stream.StreamBigNumbers(5)

    // Keep track of numbers received before cancellation
    const numbersBeforeCancel: number[] = []

    // Start consuming the stream
    const streamPromise = (async () => {
      try {
        for await (const partial of stream) {
          numbersBeforeCancel.push(partial.number)
          // Cancel after receiving the first number
          if (numbersBeforeCancel.length === 1) {
            stream.cancel()
          }
        }
      } catch (error: unknown) {
        // We expect an error after cancellation
        expect((error as Error).message).toBe('Stream was cancelled')
      }
    })()

    // Wait for the stream to complete or be cancelled
    await streamPromise

    // Verify we received exactly one number before cancellation
    expect(numbersBeforeCancel.length).toBe(1)

    // Verify that trying to get final response throws an error
    await expect(stream.getFinalResponse()).rejects.toThrow('Stream was cancelled')
  })

  it('should allow immediate cancellation', async () => {
    const stream = b.stream.StreamBigNumbers(5)

    // Cancel immediately before consuming
    stream.cancel()

    // Verify that the stream yields no values
    const numbers: number[] = []
    for await (const partial of stream) {
      numbers.push(partial.number)
    }

    expect(numbers.length).toBe(0)
    await expect(stream.getFinalResponse()).rejects.toThrow('Stream was cancelled')
  })
})
