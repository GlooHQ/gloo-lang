import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog'
import { atom, useAtom, useAtomValue, useSetAtom } from 'jotai'
import { hasClosedIntroToChecksDialogAtom, showIntroToChecksDialogAtom } from '../baml_wasm_web/EventListener'
import JsonView from 'react18-json-view'
import { CodeMirrorViewer } from './CodeMirrorViewer'

const CheckIntrosDialog: React.FC<{}> = () => {
  const [showIntroToChecksDialog, setShowIntroToChecksDialog] = useAtom(showIntroToChecksDialogAtom)
  const [hasClosedIntroToChecksDialog, setHasClosedIntroToChecksDialog] = useAtom(hasClosedIntroToChecksDialogAtom)

  const bamlExampleWithoutChecks = `
class Contact {
  email string
}`.trim()
  const bamlExampleWithChecks = `
class Contact {
  email string 
      @check(isGmail, {{ "@gmail.com" in this }})
}`.trim()

  return (
    <Dialog
      open={showIntroToChecksDialog}
      onOpenChange={(open) => {
        setShowIntroToChecksDialog(open)
        setHasClosedIntroToChecksDialog(true)
      }}
    >
      <DialogContent className=' min-h-[550px] max-h-[550px] overflow-y-auto bg-vscode-editorWidget-background flex flex-col border-vscode-textSeparator-foreground overflow-x-clip'>
        <DialogHeader className='flex flex-row gap-x-4 items-end'>
          <DialogTitle className='font-semibold'>Introduction to Checks</DialogTitle>
        </DialogHeader>
        <div className='flex flex-col gap-2 gap-y-6'>
          <p>
            It looks like you're using <code>@check</code>!
          </p>
          <p>
            You should know that <code>@check</code> will change your output JSON:
          </p>
          <CodeMirrorViewer fileContent={bamlExampleWithChecks} lang='baml' />
          <JsonView
            enableClipboard={false}
            className='bg-[#1E1E1E] px-1 py-1 rounded-sm'
            theme='a11y'
            collapseStringsAfterLength={200}
            matchesURL
            src={{
              email: {
                value: 'my.name@gmail.com',
                checks: {
                  isGmail: true,
                },
              },
            }}
          />
          <p>
            whereas without <code>@check</code>, the output JSON will look exactly like the BAML object:
          </p>
          <CodeMirrorViewer fileContent={bamlExampleWithoutChecks} lang='baml' />
          <JsonView
            enableClipboard={false}
            className='bg-[#1E1E1E] px-1 py-1 rounded-sm'
            theme='a11y'
            collapseStringsAfterLength={200}
            matchesURL
            src={{
              email: 'my.name@gmail.com',
            }}
          />
        </div>
      </DialogContent>
    </Dialog>
  )
}

export default CheckIntrosDialog
