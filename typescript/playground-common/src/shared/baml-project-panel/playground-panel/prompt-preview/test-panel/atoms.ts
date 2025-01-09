import { atom } from 'jotai'
import { type TestState } from '../../atoms'
import { atomWithStorage } from 'jotai/utils'

export interface TestHistoryEntry {
  timestamp: number
  functionName: string
  testName: string
  response: TestState
  input?: any
}

export interface TestHistoryRun {
  timestamp: number
  tests: TestHistoryEntry[]
}

export const testHistoryAtom = atomWithStorage<TestHistoryRun[]>('test-history', [])
export const selectedHistoryIndexAtom = atom<number>(0)
