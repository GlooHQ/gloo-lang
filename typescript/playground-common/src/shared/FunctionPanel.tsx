/// Content once a function has been selected.

import { Separator } from '../components/ui/separator'
import { TestCaseSelector } from './Selectors'
import { useSelections } from './hooks'
import { VSCodeDivider, VSCodePanels } from '@vscode/webview-ui-toolkit/react'
import TestCasePanel from './TestCasePanel'
import ImplPanel from './ImplPanel'
import { useContext, useEffect, useState } from 'react'
import { ASTContext } from './ASTProvider'
import TypeComponent from './TypeComponent'
import TestResultPanel from './TestResultOutcomes'
import { ScrollArea, ScrollBar } from '../components/ui/scroll-area'
import { Button } from '../components/ui/button'
import { FlaskConical } from 'lucide-react'
import clsx from 'clsx'
import { TooltipProvider } from '../components/ui/tooltip'
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from '../components/ui/resizable'

const FunctionPanel: React.FC = () => {
  const {
    selections: { showTests },
    setSelection,
  } = useContext(ASTContext)
  const { func, impl } = useSelections()

  if (!func) return <div className="flex flex-col">No function selected</div>
  const { test_results } = useSelections()
  const results = test_results ?? []

  return (
    <div
      className="flex flex-col w-full overflow-auto"
      style={{
        height: 'calc(100vh - 80px)',
      }}
    >
      <TooltipProvider>
        {/* <Allotment vertical> */}
        <div
          className={clsx('w-full flex-shrink-0 flex-grow-0', {
            'basis-[60%]': showTests && results.length > 0,
            'basis-[100%]': !showTests,
            'basis-[85%]': showTests && !(results.length > 0),
          })}
        >
          {/* <Allotment className="h-full"> */}
          <ResizablePanelGroup direction="horizontal" className="h-full">
            {impl && (
              <ResizablePanel defaultSize={50} className="px-0">
                <div className="relative h-full">
                  <ScrollArea type="always" className="flex w-full h-full pr-3 ">
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
                        <ImplPanel impl={impl} key={`${func.name.value}-${impl.name.value}`} />
                      ))}
                    </VSCodePanels>
                  </ScrollArea>
                </div>
              </ResizablePanel>
            )}
            <ResizableHandle withHandle={true} className="bg-vscode-panel-border" />
            <ResizablePanel minSize={50} className="pl-2 pr-0.5" hidden={!showTests}>
              {/* <Allotment.Pane className="pl-2 pr-0.5" minSize={200} visible={showTests}> */}
              <div className="flex flex-col h-full overflow-y-auto overflow-x-clip">
                {/* On windows this scroll area extends beyond the wanted width, so we just use a normal scrollbar here vs using ScrollArea*/}
                <TestCasePanel func={func} />
              </div>
            </ResizablePanel>
          </ResizablePanelGroup>
          {/* </Allotment> */}
        </div>
        <div
          className={clsx('py-2 border-t h-fit border-vscode-textSeparator-foreground', {
            flex: showTests,
            hidden: !showTests,
          })}
        >
          <div className="w-full h-full">
            <TestResultPanel />
          </div>
        </div>
      </TooltipProvider>
    </div>
  )
}

export default FunctionPanel
