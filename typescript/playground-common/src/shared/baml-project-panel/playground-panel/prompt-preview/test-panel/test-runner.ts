import type { WasmFunctionResponse, WasmTestResponse } from '@gloo-ai/baml-schema-wasm-web'
import { atom, useAtomValue, useSetAtom } from 'jotai'
import { useAtomCallback } from 'jotai/utils'
import { useCallback, useRef } from 'react'
import { ctxAtom, runtimeAtom, wasmAtom } from '../../../atoms'
import {
  type TestState,
  areTestsRunningAtom,
  selectedFunctionAtom,
  selectedTestcaseAtom,
  testCaseAtom,
} from '../../atoms'
import { isClientCallGraphEnabledAtom } from '../../preview-toolbar'
import { findMediaFile } from '../media-utils'
import { type TestHistoryRun, selectedHistoryIndexAtom, testHistoryAtom } from './atoms'

// Map to store abort controllers for individual tests
export const testAbortControllersAtom = atom<Map<string, AbortController>>(new Map())

// Helper to generate a unique key for a test
const getTestKey = (test: { functionName: string; testName: string }) => `${test.functionName}:${test.testName}`

export const useRunTests = (maxBatchSize = 5) => {
  const { rt } = useAtomValue(runtimeAtom)
  const ctx = useAtomValue(ctxAtom)
  const wasm = useAtomValue(wasmAtom)
  const setSelectedTestcase = useSetAtom(selectedTestcaseAtom)
  const setSelectedFunction = useSetAtom(selectedFunctionAtom)
  const setIsClientCallGraphEnabled = useSetAtom(isClientCallGraphEnabledAtom)
  const setTestAbortControllers = useSetAtom(testAbortControllersAtom)
  const testAbortControllersRef = useRef<Map<string, AbortController>>(new Map())

  const runTests = useAtomCallback(
    useCallback(
      async (get, set, tests: { functionName: string; testName: string }[]) => {
        // Clear any existing abort controllers
        testAbortControllersRef.current.clear()
        setTestAbortControllers(new Map())

        // Create a new history run
        const historyRun: TestHistoryRun = {
          timestamp: Date.now(),
          tests: tests.map((test) => ({
            timestamp: Date.now(),
            functionName: test.functionName,
            testName: test.testName,
            response: { status: 'running' },
            input: get(testCaseAtom(test))?.tc.inputs, // Store input
          })),
        }

        setIsClientCallGraphEnabled(false)

        set(testHistoryAtom, (prev) => [historyRun, ...prev])
        set(selectedHistoryIndexAtom, 0)

        const setState = (test: { functionName: string; testName: string }, update: TestState) => {
          set(testHistoryAtom, (prev) => {
            const newHistory = [...prev]
            const currentRun = newHistory[0]
            if (!currentRun) return prev

            const testIndex = currentRun.tests.findIndex(
              (t) => t.functionName === test.functionName && t.testName === test.testName,
            )
            if (testIndex === -1) return prev

            const existingTest = currentRun.tests[testIndex]
            if (!existingTest) return prev

            currentRun.tests[testIndex] = {
              ...existingTest,
              response: update,
              timestamp: Date.now(),
              functionName: existingTest.functionName,
              testName: existingTest.testName,
            }
            return newHistory
          })
        }

        const runTest = async (test: { functionName: string; testName: string }) => {
          const testKey = getTestKey(test)
          const controller = new AbortController()
          testAbortControllersRef.current.set(testKey, controller)
          setTestAbortControllers(new Map(testAbortControllersRef.current))

          try {
            // Check if aborted before starting test
            if (controller.signal.aborted) {
              setState(test, { status: 'error', message: 'Test cancelled' })
              return
            }

            const testCase = get(testCaseAtom(test))
            if (!rt || !ctx || !testCase || !wasm) {
              setState(test, { status: 'error', message: 'Missing required dependencies' })
              console.error('Missing required dependencies')
              return
            }

            const startTime = performance.now()
            setState(test, { status: 'running' })

            // Create a promise that rejects when the abort signal is triggered
            const abortPromise = new Promise((_, reject) => {
              controller.signal.addEventListener('abort', () => {
                reject(new Error('Test cancelled'))
              })
            })

            // Race between the test execution and abort signal
            const result = (await Promise.race([
              new Promise(async (resolve, reject) => {
                try {
                  // Check abort signal before starting test
                  if (controller.signal.aborted) {
                    reject(new Error('Test cancelled'))
                    return
                  }

                  const result = await testCase.fn.run_test(
                    rt,
                    testCase.tc.name,
                    (partial: WasmFunctionResponse) => {
                      if (!controller.signal.aborted) {
                        setState(test, { status: 'running', response: partial })
                      }
                    },
                    findMediaFile,
                  )
                  resolve(result)
                } catch (e) {
                  reject(e)
                }
              }),
              abortPromise,
            ])) as WasmTestResponse

            // Check if aborted after test completion
            if (controller.signal.aborted) {
              setState(test, { status: 'error', message: 'Test cancelled' })
              return
            }

            const endTime = performance.now()
            const response_status = result.status()
            const responseStatusMap = {
              [wasm.TestStatus.Passed]: 'passed',
              [wasm.TestStatus.LLMFailure]: 'llm_failed',
              [wasm.TestStatus.ParseFailure]: 'parse_failed',
              [wasm.TestStatus.ConstraintsFailed]: 'constraints_failed',
              [wasm.TestStatus.AssertFailed]: 'assert_failed',
              [wasm.TestStatus.UnableToRun]: 'error',
              [wasm.TestStatus.FinishReasonFailed]: 'error',
            } as const

            // Ensure response_status is a valid key
            const status = responseStatusMap[response_status as keyof typeof responseStatusMap] || 'error'

            setState(test, {
              status: 'done',
              response: result,
              response_status: status,
              latency_ms: endTime - startTime,
            })
          } catch (e) {
            console.log('test error!')
            console.error(e)

            // Special handling for cancellation
            if (e instanceof Error && e.message === 'Test cancelled') {
              setState(test, {
                status: 'error',
                message: 'Test cancelled',
              })
            } else {
              setState(test, {
                status: 'error',
                message: e instanceof Error ? e.message : 'Unknown error',
              })
            }
          } finally {
            // Clean up the abort controller for this test
            testAbortControllersRef.current.delete(testKey)
            setTestAbortControllers(new Map(testAbortControllersRef.current))
          }
        }

        const run = async () => {
          try {
            // Create batches of tests to run
            const batches: { functionName: string; testName: string }[][] = []
            for (let i = 0; i < tests.length; i += maxBatchSize) {
              batches.push(tests.slice(i, i + maxBatchSize))
            }

            if (tests.length == 0) {
              console.error('No tests found')
              return
            }

            const firstTest = get(testCaseAtom(tests[0]))
            if (firstTest) {
              setSelectedFunction(firstTest.fn.name)
              setSelectedTestcase(firstTest.tc.name)
            } else {
              console.error("Invalid test found, so won't select this test case in the prompt preview", tests[0])
            }

            // Run each batch
            for (const batch of batches) {
              // TODO: parallelize when we fix wasm issues with runtime undefined after multiple runs
              for (const test of batch) {
                setState(test, { status: 'queued' })
                await runTest(test)
              }
            }
          } finally {
            // Clear all abort controllers
            testAbortControllersRef.current.clear()
            setTestAbortControllers(new Map())
          }
        }

        set(areTestsRunningAtom, true)
        await run().finally(() => {
          set(areTestsRunningAtom, false)
        })
      },
      [maxBatchSize, rt, ctx, wasm],
    ),
  )

  const cancelTest = useCallback(
    (test: { functionName: string; testName: string }) => {
      const testKey = getTestKey(test)
      const controller = testAbortControllersRef.current.get(testKey)
      if (controller) {
        controller.abort()
        testAbortControllersRef.current.delete(testKey)
        setTestAbortControllers(new Map(testAbortControllersRef.current))
      }
    },
    [setTestAbortControllers],
  )

  const cancelAllTests = useCallback(() => {
    // Abort all running tests
    testAbortControllersRef.current.forEach((controller) => controller.abort())
    testAbortControllersRef.current.clear()
    setTestAbortControllers(new Map())
  }, [setTestAbortControllers])

  return { setRunningTests: runTests, cancelTest, cancelAllTests }
}
