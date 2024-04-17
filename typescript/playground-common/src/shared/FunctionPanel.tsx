/// Content once a function has been selected.

import { Separator } from '../components/ui/separator'
import { TestCaseSelector } from './Selectors'
import { useSelections } from './hooks'
import { VSCodeDivider, VSCodePanels } from '@vscode/webview-ui-toolkit/react'
import TestCasePanel from './TestCasePanel'
import ImplPanel from './ImplPanel'
import { createRef, useContext, useEffect, useId, useImperativeHandle, useRef, useState } from 'react'
import { ASTContext } from './ASTProvider'
import TypeComponent from './TypeComponent'
import TestResultPanel from './TestResultOutcomes'
import { ScrollArea, ScrollBar } from '../components/ui/scroll-area'
import { Button } from '../components/ui/button'
import { FlaskConical } from 'lucide-react'
import clsx from 'clsx'
import { TooltipProvider } from '../components/ui/tooltip'
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from '../components/ui/resizable'
import { ImperativePanelHandle } from 'react-resizable-panels'
import { TestResult } from '@baml/common'

function getTopPanelSize(showTests: boolean, test_results: TestResult[] | undefined): number {
  if (showTests) {
    if (test_results && test_results.length > 0) {
      return 40
    } else {
      return 85
    }
  }
  return 100
}

const FunctionPanel: React.FC = () => {
  const {
    selections: { showTests },
    setSelection,
  } = useContext(ASTContext)
  const { func, impl } = useSelections()
  const { test_results } = useSelections()
  const results = test_results ?? []
  const id = useId()
  const refs = useRef()
  const testResultId = test_results ? test_results[0]?.status : ''
  const ref = createRef<ImperativePanelHandle>()

  useEffect(() => {
    let topPanelSize = getTopPanelSize(showTests, test_results)
    if (ref.current) {
      ref.current.resize(topPanelSize)
    }
  }, [showTests, testResultId])

  if (!func)
    return <div className="flex flex-col">No function selected. Create or select a function to get started</div>

  let impls = <div />
  if (!impl) {
    impls = <div />
  } else if (func.impls.length === 1) {
    impls = <ImplPanel showTab={false} impl={func.impls[0]} />
  } else {
    impls = (
      <VSCodePanels
        activeid={`tab-${func.name.value}-${impl.name.value}`}
        onChange={(e) => {
          const selected: string | undefined = (e.target as any)?.activetab?.id
          if (selected && selected.startsWith(`tab-${func.name.value}-`)) {
            setSelection(undefined, undefined, selected.split('-', 3)[2], undefined, undefined)
          }
        }}
      >
        {func.impls.map((impl) => (
          <ImplPanel showTab={true} impl={impl} key={`${func.name.value}-${impl.name.value}`} />
        ))}
      </VSCodePanels>
    )
  }

  let topPanelSize = getTopPanelSize(showTests, test_results)

  return (
    <div
      className="flex flex-col w-full overflow-auto"
      style={{
        height: 'calc(100vh - 80px)',
      }}
    >
      <TooltipProvider>
        <ResizablePanelGroup direction="vertical" className="h-full">
          <ResizablePanel id="top-panel" ref={ref} className="flex w-full " defaultSize={topPanelSize}>
            <div className="w-full">
              <ResizablePanelGroup direction="horizontal" className="h-full">
                {impl && (
                  <ResizablePanel defaultSize={50} className="px-0">
                    <div className="relative h-full">
                      <ScrollArea type="always" className="flex w-full h-full pr-3 ">
                        {impls}
                      </ScrollArea>
                    </div>
                  </ResizablePanel>
                )}
                <ResizableHandle withHandle={false} className="bg-vscode-panel-border" />
                <ResizablePanel minSize={30} className="pl-2 pr-0.5" hidden={!showTests}>
                  {/* <Allotment.Pane className="pl-2 pr-0.5" minSize={200} visible={showTests}> */}
                  <div className="flex flex-col h-full overflow-y-auto overflow-x-clip">
                    {/* On windows this scroll area extends beyond the wanted width, so we just use a normal scrollbar here vs using ScrollArea*/}
                    <TestCasePanel func={func} />
                  </div>
                </ResizablePanel>
              </ResizablePanelGroup>

              {/* </Allotment> */}
            </div>
          </ResizablePanel>
          <ResizableHandle withHandle={false} className="bg-vscode-panel-border" />
          <ResizablePanel minSize={10} className="px-0 overflow-y-auto">
            <div
              className={clsx('py-2 border-t h-full border-vscode-textSeparator-foreground', {
                flex: showTests,
                hidden: !showTests,
              })}
            >
              <div className="w-full h-full">
                <TestResultPanel />
              </div>
            </div>
          </ResizablePanel>
        </ResizablePanelGroup>
      </TooltipProvider>
    </div>
  )
}

export default FunctionPanel
