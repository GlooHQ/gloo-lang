/* eslint-disable @typescript-eslint/no-floating-promises */
import { useAtomValue, useSetAtom } from 'jotai'
import { Play, Square } from 'lucide-react'
import { useEffect, useRef } from 'react'
import { Button } from '~/components/ui/button'
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '~/components/ui/tooltip'
import { cn } from '~/lib/utils'
import { type TestState, selectedItemAtom, testCaseResponseAtom } from '../../../atoms'
import { FunctionTestName } from '../../../function-test-name'
import { type TestHistoryRun } from '../atoms'
import { useRunTests } from '../test-runner'
import { getStatus } from '../testStateUtils'
import { ResponseRenderer } from './ResponseRenderer'
import { TestStatus } from './TestStatus'

export const CardView = ({ currentRun }: { currentRun: TestHistoryRun }) => {
  return (
    <div className='space-y-4'>
      {currentRun.tests.map((test, index) => (
        <TestResult
          key={index}
          testId={{
            functionName: test.functionName,
            testName: test.testName,
          }}
          historicalResponse={test.response}
        />
      ))}
    </div>
  )
}
interface TestId {
  functionName: string
  testName: string
}

const TestResult = ({ testId, historicalResponse }: { testId: TestId; historicalResponse?: TestState }) => {
  const response = useAtomValue(testCaseResponseAtom(testId))
  const displayResponse = historicalResponse || response
  const { setRunningTests, cancelTest } = useRunTests()
  const setSelectedItem = useSetAtom(selectedItemAtom)
  const selectedItem = useAtomValue(selectedItemAtom)
  const cardRef = useRef<HTMLDivElement>(null)

  const isSelected = selectedItem?.[0] === testId.functionName && selectedItem?.[1] === testId.testName

  useEffect(() => {
    if (isSelected && cardRef.current) {
      cardRef.current.scrollIntoView({
        behavior: 'smooth',
        block: 'nearest',
      })
    }
  }, [isSelected])

  if (!displayResponse) {
    console.log('no display response')
    return null
  }

  return (
    <div
      ref={cardRef}
      className={cn(
        'flex cursor-pointer flex-col gap-2 rounded-lg border p-3 transition-colors hover:bg-muted/70 dark:bg-muted/20',
        isSelected && 'border-purple-500/20 shadow-sm dark:border-purple-900/30 dark:bg-muted/90',
      )}
      onClick={() => setSelectedItem(testId.functionName, testId.testName)}
    >
      <div className='flex gap-2 justify-between items-center'>
        <div className='flex gap-2 items-center'>
          <TooltipProvider>
            <Tooltip delayDuration={0}>
              <TooltipTrigger asChild>
                {displayResponse.status === 'running' ? (
                  <Button
                    variant='ghost'
                    size='icon'
                    className='w-6 h-6 shrink-0'
                    onClick={(e) => {
                      e.stopPropagation()
                      cancelTest(testId)
                    }}
                  >
                    <Square className='w-4 h-4 text-red-400 fill-red-400' />
                  </Button>
                ) : (
                  <Button
                    variant='ghost'
                    size='icon'
                    className='w-6 h-6 shrink-0'
                    onClick={(e) => {
                      e.stopPropagation()
                      setRunningTests([testId])
                    }}
                  >
                    <Play className='w-4 h-4' fill='#a855f7' stroke='#a855f7' />
                  </Button>
                )}
              </TooltipTrigger>
              <TooltipContent>
                <p>{displayResponse.status === 'running' ? 'Cancel test' : 'Re-run test'}</p>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
          <FunctionTestName functionName={testId.functionName} testName={testId.testName} selected={isSelected} />
        </div>
        <TestStatus status={displayResponse.status} finalState={getStatus(displayResponse)} />
      </div>

      {displayResponse.status === 'running' && <ResponseRenderer response={displayResponse.response} />}

      {displayResponse.status === 'done' && (
        <ResponseRenderer response={displayResponse.response} status={displayResponse.response_status} />
      )}

      {displayResponse.status === 'error' && (
        <div className='mt-2 text-xs text-red-500'>Error: {displayResponse.message}</div>
      )}
    </div>
  )
}
