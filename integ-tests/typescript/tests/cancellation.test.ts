import { BamlClientFinishReasonError, BamlValidationError } from '@boundaryml/baml'
import { b } from './test-setup'

describe('Cancellation Tests', () => {
  it('should properly cancel a stream', async () => {
    // Start a streaming operation
    const stream = b.stream.StreamBigNumbers(5)
    stream.cancel()

    const request = await b.TestAws('hi')
    // request.cancel()

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
        // We expect a BamlClientFinishReasonError
        if (error instanceof BamlClientFinishReasonError) {
          expect(error.message).toContain('Stream was cancelled')
          expect(error.prompt).toBe('')
          expect(error.raw_output).toBe('')
          expect(error.finish_reason).toBe('Stream was cancelled')
        } else {
          throw error
        }
      }
    })()

    // Wait for the stream to complete or be cancelled
    await streamPromise

    // Verify we received exactly one number before cancellation
    expect(numbersBeforeCancel.length).toBe(1)

    // Verify that trying to get final response throws a BamlClientFinishReasonError
    try {
      await stream.getFinalResponse()
      expect(false).toBe(true) // This line should not be reached
    } catch (error: unknown) {
      if (error instanceof BamlClientFinishReasonError) {
        expect(error.message).toContain('Stream was cancelled')
        expect(error.prompt).toBe('')
        expect(error.raw_output).toBe('')
        expect(error.finish_reason).toBe('Stream was cancelled')
      } else {
        throw error
      }
    }
  })

  it('should allow immediate stream cancellation', async () => {
    const stream = b.stream.StreamBigNumbers(5)

    // Cancel immediately before consuming
    stream.cancel()

    // Verify that the stream yields no values and throws BamlClientFinishReasonError
    const numbers: number[] = []
    try {
      for await (const partial of stream) {
        numbers.push(partial.number)
      }
      expect(false).toBe(true) // This line should not be reached
    } catch (error: unknown) {
      if (error instanceof BamlClientFinishReasonError) {
        expect(error.message).toContain('Stream was cancelled')
        expect(error.prompt).toBe('')
        expect(error.raw_output).toBe('')
        expect(error.finish_reason).toBe('Stream was cancelled')
      } else {
        throw error
      }
    }

    expect(numbers.length).toBe(0)

    // Verify final response also throws BamlClientFinishReasonError
    try {
      await stream.getFinalResponse()
      expect(false).toBe(true) // This line should not be reached
    } catch (error: unknown) {
      if (error instanceof BamlClientFinishReasonError) {
        expect(error.message).toContain('Stream was cancelled')
        expect(error.prompt).toBe('')
        expect(error.raw_output).toBe('')
        expect(error.finish_reason).toBe('Stream was cancelled')
      } else {
        throw error
      }
    }
  })

  it('should handle cancellation during stream consumption', async () => {
    const stream = b.stream.StreamBigNumbers(10)
    const numbers: number[] = []
    let cancelledAfterCount = 0

    try {
      for await (const partial of stream) {
        numbers.push(partial.number)
        if (numbers.length === 3) {
          cancelledAfterCount = numbers.length
          stream.cancel()
        }
      }
      expect(false).toBe(true) // This line should not be reached
    } catch (error: unknown) {
      if (error instanceof BamlClientFinishReasonError) {
        expect(error.message).toContain('Stream was cancelled')
        expect(error.prompt).toBe('')
        expect(error.raw_output).toBe('')
        expect(error.finish_reason).toBe('Stream was cancelled')
      } else {
        throw error
      }
    }

    expect(cancelledAfterCount).toBe(3)
    expect(numbers.length).toBe(3)

    // Verify final response throws BamlClientFinishReasonError
    try {
      await stream.getFinalResponse()
      expect(false).toBe(true) // This line should not be reached
    } catch (error: unknown) {
      if (error instanceof BamlClientFinishReasonError) {
        expect(error.message).toContain('Stream was cancelled')
        expect(error.prompt).toBe('')
        expect(error.raw_output).toBe('')
        expect(error.finish_reason).toBe('Stream was cancelled')
      } else {
        throw error
      }
    }
  })

  it('should handle validation errors correctly', async () => {
    try {
      // Attempt to call with invalid input that will trigger validation error
      await b.stream.StreamBigNumbers(-1)
      expect(false).toBe(true) // This line should not be reached
    } catch (error: unknown) {
      if (error instanceof BamlValidationError) {
        expect(error.prompt).toBeDefined()
        expect(error.raw_output).toBeDefined()
        expect(error.message).toContain('BamlValidationError')
      } else {
        throw error
      }
    }
  })
})
