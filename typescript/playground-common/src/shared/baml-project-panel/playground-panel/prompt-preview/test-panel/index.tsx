/* eslint-disable @typescript-eslint/no-floating-promises */
import { useAtom, useAtomValue } from 'jotai'
import { ScrollArea } from '~/components/ui/scroll-area'
import { selectedHistoryIndexAtom, testHistoryAtom } from './atoms'
import { TestPanelViewType, testPanelViewTypeAtom } from './components/atoms'
import { CardView } from './components/CardView'
import { SimpleCardView } from './components/SimpleCardView'
import { TabularView } from './components/TabularView'
import { TestMenu } from './components/TestMenu'
import { ClientGraphView } from './components/ClientGraphView'
import { ErrorBoundary } from 'react-error-boundary'
import { isClientCallGraphEnabledAtom } from '../../preview-toolbar'

const TestPanel = () => {
  const [selectedHistoryIndex, setSelectedHistoryIndex] = useAtom(selectedHistoryIndexAtom)
  const testHistory = useAtomValue(testHistoryAtom)
  const viewType = useAtomValue(testPanelViewTypeAtom)
  const isClientCallGraphEnabled = useAtomValue(isClientCallGraphEnabledAtom)

  if (isClientCallGraphEnabled) {
    return <ClientGraphView />
  }

  // TODO: still render the client graph view even if no tests are running.
  if (testHistory.length === 0) {
    return <div className="p-4 text-muted-foreground">No tests running</div>
  }

  const currentRun = testHistory[selectedHistoryIndex]

  const renderView = () => {
    switch (viewType) {
      case TestPanelViewType.TABULAR:
        return <TabularView currentRun={currentRun} />
      case TestPanelViewType.CARD_SIMPLE:
        return <SimpleCardView currentRun={currentRun} />
      case TestPanelViewType.CARD_EXPANDED:
        return <CardView currentRun={currentRun} />
      case TestPanelViewType.CLIENT_GRAPH:
        return <ClientGraphView />
      default:
        return null
    }
  }

  return (
    <>
      <div className="px-1 pt-2">
        <ErrorBoundary
          fallback={<div>Error rendering</div>}
          onReset={() => {
            // Reset any state that may have caused the error
            window.location.reload()
          }}
          resetKeys={[viewType, currentRun]}
        >
          <TestMenu />
        </ErrorBoundary>
      </div>

      <ScrollArea className="relative flex-1 p-0" type="always">
        {currentRun && (
          <div className="mb-1 text-xs text-muted-foreground/50">{new Date(currentRun.timestamp).toLocaleString()}</div>
        )}
        <ErrorBoundary
          fallback={<div>Error rendering view</div>}
          onReset={() => {
            // Reset any state that may have caused the error
            window.location.reload()
          }}
          resetKeys={[viewType, currentRun]}
        >
          {renderView()}
        </ErrorBoundary>
      </ScrollArea>
    </>
  )
}

export default TestPanel
