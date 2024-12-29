import { atom } from 'jotai'
import { type TestState } from '../../atoms'

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

export const testHistoryAtom = atom<TestHistoryRun[]>([])
export const selectedHistoryIndexAtom = atom<number>(0)
