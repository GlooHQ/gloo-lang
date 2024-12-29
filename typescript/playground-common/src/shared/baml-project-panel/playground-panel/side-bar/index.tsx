/* eslint-disable @typescript-eslint/no-floating-promises */
import { Input } from '@/components/ui/input'
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover'
import { ResizablePanel, ResizablePanelGroup } from '@/components/ui/resizable'
import { cn } from '@/lib/utils'
import { Dialog, DialogContent } from '@radix-ui/react-dialog'
import { atom, useAtomValue, useSetAtom } from 'jotai'
import {
  AlertTriangle,
  CheckCircle2,
  ChevronLeft,
  ChevronRight,
  FlaskConical,
  Loader,
  Play,
  Search,
  Settings,
  XCircle,
} from 'lucide-react'
import { AnimatePresence, motion } from 'motion/react'
import * as React from 'react'
import { Button } from '~/components/ui/button'
import { runtimeStateAtom, selectedItemAtom } from '../atoms'
import { selectedHistoryIndexAtom, testHistoryAtom } from '../prompt-preview/test-panel/atoms'
import { useRunTests } from '../prompt-preview/test-panel/test-runner'
import { getStatus } from '../prompt-preview/test-panel/testStateUtils'
import EnvVars from './env-vars'

interface FunctionData {
  name: string
  tests: string[]
}

interface SidepanelProps {
  functions: FunctionData[]
  searchTerm: string
}
const functionsAtom = atom((get) => {
  const runtimeState = get(runtimeStateAtom)
  if (runtimeState === undefined) {
    return []
  }
  return runtimeState.functions.map((f) => ({
    name: f.name,
    tests: f.test_cases.map((t) => t.name),
  }))
})

export default function CustomSidebar() {
  const functions = useAtomValue(functionsAtom)
  const [searchTerm, setSearchTerm] = React.useState('')
  const [showEnvDialog, setShowEnvDialog] = React.useState(false)
  const [isOpen, setIsOpen] = React.useState(true)
  const { setRunningTests } = useRunTests()

  const filteredFunctions = functions.filter(
    (func) =>
      func.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
      func.tests.some((test) => test.toLowerCase().includes(searchTerm.toLowerCase())),
  )

  const handleRunFilteredTests = () => {
    const testsToRun = filteredFunctions.flatMap((func) =>
      func.tests.map((test) => ({
        functionName: func.name,
        testName: test,
      })),
    )
    setRunningTests(testsToRun)
  }

  if (functions.length === 0 || (functions.length === 1 && functions[0]?.tests.length === 1)) {
    return <></>
  }

  return (
    <div className='flex relative h-full'>
      <Button
        onClick={() => setIsOpen(!isOpen)}
        variant='ghost'
        size='sm'
        className={cn(
          'absolute -left-6 top-1/2 z-10 p-0 w-8 h-12 -translate-y-1/2 hover:bg-muted',
          isOpen ? 'rounded-l' : 'rounded',
        )}
      >
        <ChevronLeft className={cn('w-4 h-4 transition-transform duration-200', isOpen ? 'rotate-180' : '')} />
        <span className='sr-only'>Toggle sidebar</span>
      </Button>
      <div
        className={cn(
          'flex flex-col h-full border-l transition-all duration-200 border-border bg-background/50',
          isOpen ? 'opacity-100 w-[160px] min-w-[160px]' : 'w-8 opacity-100 min-w-8',
        )}
      >
        <ResizablePanelGroup direction='vertical'>
          <ResizablePanel defaultSize={75}>
            <div className='flex h-[60px] items-center px-4 text-xs'>
              <div className='relative w-full'>
                <div className='absolute inset-0 -m-0.5 rounded-md transition-all' />
                <div className='flex relative items-center'>
                  <Search className='absolute left-2 top-1/2 w-3 h-3 text-gray-400 -translate-y-1/2' />
                  <Input
                    placeholder='Filter...'
                    value={searchTerm}
                    onChange={(e) => setSearchTerm(e.target.value)}
                    className='flex px-8 py-2 w-full h-9 text-xs rounded-md border border-input bg-background focus:outline-none focus:ring-2 focus:ring-ring'
                  />
                </div>
              </div>
            </div>
            <div className='overflow-auto flex-1'>
              <div className='px-2'>
                {searchTerm && filteredFunctions.length > 0 && (
                  <Button
                    variant='ghost'
                    size='sm'
                    onClick={handleRunFilteredTests}
                    className='flex justify-between items-center mb-2 w-full'
                  >
                    <span>Run tests below</span>
                    <Play className='ml-2 w-3 h-3' />
                  </Button>
                )}
                <TreeView functions={filteredFunctions} searchTerm={searchTerm} />
              </div>
            </div>
          </ResizablePanel>
          {/* <ResizableHandle withHandle />
          <ResizablePanel defaultSize={25}>
            <ScrollArea className="h-full" type="always">
              <EnvVars />
            </ScrollArea>
          </ResizablePanel> */}
        </ResizablePanelGroup>
      </div>
    </div>
  )
}

interface FunctionItemProps {
  label: string
  tests: string[]
  isLast?: boolean
  isSelected?: boolean
  searchTerm?: string
}

function FunctionItem({ label, tests, isLast = false, isSelected = false, searchTerm = '' }: FunctionItemProps) {
  const [isOpen, setIsOpen] = React.useState(true)
  const { setRunningTests } = useRunTests()
  const setSelectedItem = useSetAtom(selectedItemAtom)
  const selectedItem = useAtomValue(selectedItemAtom)
  const handleClick = (e: React.MouseEvent) => {
    e.stopPropagation()
    setIsOpen(!isOpen)
  }

  const handleRunAll = (e: React.MouseEvent) => {
    e.stopPropagation()
    const testsToRun = tests.map((test) => ({
      functionName: label,
      testName: test,
    }))

    // select the first test in the list if we ran tests for
    // this function.
    if (tests.length > 0) {
      // this causes a "recursive use of object" error since
      // we execute a wasm function in parallel as tests start running.
      // figure out how to fix this.
      //  setSelectedItem(label, tests[0]!);
    }

    setRunningTests(testsToRun)
  }

  const highlightText = (text: string) => {
    if (!searchTerm) return text

    const parts = text.split(new RegExp(`(${searchTerm})`, 'gi'))
    return (
      <span>
        {parts.map((part, i) =>
          part.toLowerCase() === searchTerm.toLowerCase() ? (
            <span key={i} className='bg-yellow-200 dark:bg-yellow-900'>
              {part}
            </span>
          ) : (
            part
          ),
        )}
      </span>
    )
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: -10 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -10 }}
      transition={{ duration: 0.2 }}
    >
      <div
        className={cn(
          'flex relative items-center px-1 py-1 -mx-2 transition-colors cursor-pointer hover:bg-muted',
          isSelected ? 'font-bold text-purple-400' : 'text-muted-foreground',
        )}
        onClick={handleClick}
      >
        <div className='flex flex-1 items-center min-w-0'>
          <motion.div
            initial={false}
            animate={{ rotate: isOpen ? 90 : 0 }}
            transition={{ duration: 0.2 }}
            className='mr-1'
          >
            <ChevronRight className='w-3 h-3' />
          </motion.div>
          <span className='ml-1 font-mono text-xs truncate'>{highlightText(label)}</span>
        </div>
        <Button variant='ghost' size='sm' className='p-0 w-6 h-6' onClick={handleRunAll}>
          <Play className='w-3 h-3' />
        </Button>
      </div>
      <AnimatePresence initial={false}>
        {isOpen && (
          <motion.div
            initial={{ opacity: 0, height: 0 }}
            animate={{ opacity: 1, height: 'auto' }}
            exit={{ opacity: 0, height: 0 }}
            transition={{ duration: 0.2 }}
            className='overflow-hidden ml-4'
          >
            {tests.map((test, index) => (
              <TestItem
                key={index}
                label={test}
                isSelected={selectedItem?.[0] === label && selectedItem?.[1] === test}
                searchTerm={searchTerm}
                functionName={label}
              />
            ))}
          </motion.div>
        )}
      </AnimatePresence>
    </motion.div>
  )
}

interface TestItemProps {
  label: string
  isLast?: boolean
  isSelected?: boolean
  searchTerm?: string
  functionName: string
}

function TestItem({ label, isLast = false, isSelected = false, searchTerm = '', functionName }: TestItemProps) {
  const testHistory = useAtomValue(testHistoryAtom)
  const selectedIndex = useAtomValue(selectedHistoryIndexAtom)
  const { setRunningTests } = useRunTests()

  const currentRun = testHistory[selectedIndex]
  const testResult = currentRun?.tests.find((t) => t.functionName === functionName && t.testName === label)

  // TODO: coalesce with the other status in TestStatus.tsx
  const getStatusIcon = () => {
    if (!testResult) return <FlaskConical className='w-3 h-3' />

    const status = testResult.response.status
    const finalState = getStatus(testResult.response)

    if (status === 'running') return <Loader className='w-3 h-3' />
    if (status === 'error') return <XCircle className='w-3 h-3 text-red-500' />
    if (status === 'done') {
      if (finalState === 'passed') return <CheckCircle2 className='w-3 h-3 text-green-500' />
      if (finalState === 'constraints_failed') return <AlertTriangle className='w-3 h-3 text-yellow-500' />
      return <XCircle className='w-3 h-3 text-red-500' />
    }
    return <FlaskConical className='w-3 h-3' />
  }

  const setSelectedItem = useSetAtom(selectedItemAtom)

  const handleClick = (e: React.MouseEvent) => {
    e.stopPropagation()
    setSelectedItem(functionName, label)
  }

  const handleRunTest = (e: React.MouseEvent) => {
    e.stopPropagation()
    setRunningTests([{ functionName, testName: label }])
  }

  const highlightText = (text: string) => {
    if (!searchTerm) return text

    const parts = text.split(new RegExp(`(${searchTerm})`, 'gi'))
    return (
      <span>
        {parts.map((part, i) =>
          part.toLowerCase() === searchTerm.toLowerCase() ? (
            <span key={i} className='bg-yellow-200 dark:bg-yellow-900'>
              {part}
            </span>
          ) : (
            part
          ),
        )}
      </span>
    )
  }

  return (
    <motion.div
      initial={{ opacity: 0, y: -10 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -10 }}
      transition={{ duration: 0.2 }}
      className='ml-2'
    >
      <div
        className={cn(
          'flex relative items-center px-1 py-1 -mx-2 transition-colors cursor-pointer group',
          isSelected ? 'border-l-4 border-l-purple-500 bg-purple-500/10' : 'hover:bg-muted',
          isSelected ? 'text-foreground' : 'text-muted-foreground',
        )}
        onClick={handleClick}
      >
        <div className='flex flex-1 items-center min-w-0'>
          {getStatusIcon()}
          <span className='ml-1 font-mono text-xs truncate'>{highlightText(label)}</span>
        </div>
        <Button
          variant='ghost'
          size='sm'
          className='p-0 w-6 h-6 opacity-0 transition-opacity group-hover:opacity-100'
          onClick={handleRunTest}
        >
          <Play className='w-3 h-3' />
        </Button>
      </div>
    </motion.div>
  )
}

interface TreeViewProps {
  functions: FunctionData[]
  searchTerm: string
}

function TreeView({ functions, searchTerm }: TreeViewProps) {
  const selectedItem = useAtomValue(selectedItemAtom)

  return (
    <div className='space-y-1'>
      {functions.map((func, index) => (
        <FunctionItem
          key={func.name}
          label={func.name}
          tests={func.tests}
          isLast={index === functions.length - 1}
          isSelected={selectedItem?.[0] === func.name}
          searchTerm={searchTerm}
        />
      ))}
    </div>
  )
}
