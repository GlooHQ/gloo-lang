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

// TODO: make this persistent, but make sure to serialize the wasm objects properly
export const testHistoryAtom = atom<TestHistoryRun[]>([])
export const selectedHistoryIndexAtom = atom<number>(0)
