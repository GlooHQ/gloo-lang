import { useAtomValue } from 'jotai'
import { ctxAtom, runtimeAtom } from '../../atoms'
import { selectionAtom } from '../atoms'
import useSWR from 'swr'
import { Loader } from './components'
import { ErrorMessage } from './components'
import { WithCopyButton } from './components'
import { findMediaFile } from './media-utils'

// const rawCurlAtomAsync = atom(async (get) => {
//   const wasm = get(wasmAtom)
//   const runtime = get(selectedRuntimeAtom)
//   const func = get(selectedFunctionAtom)
//   const test_case = get(selectedTestCaseAtom)
//   const orch_index = get(orchIndexAtom)
//   if (!wasm || !runtime || !func || !test_case) {
//     return null
//   }

//   const streamCurl = get(streamCurlAtom)
//   const expandImages = get(expandImagesAtom)

//   const wasmCallContext = new wasm.WasmCallContext()
//   wasmCallContext.node_index = orch_index

//   return await func.render_raw_curl_for_test(
//     runtime,
//     test_case.name,
//     wasmCallContext,
//     streamCurl,
//     expandImages,
//     async (path: string) => {
//       return await vscode.readFile(path)
//     },
//   )
// })

export const PromptPreviewCurl = () => {
  const rt = useAtomValue(runtimeAtom).rt
  const ctx = useAtomValue(ctxAtom)
  const { selectedFn, selectedTc } = useAtomValue(selectionAtom)

  const {
    data: curl,
    error,
    isLoading,
  } = useSWR(rt && ctx && selectedFn && selectedTc ? ['curl', rt, ctx, selectedFn, selectedTc] : null, async () => {
    if (!selectedFn || !rt || !selectedTc || !ctx) {
      throw new Error('Not initialized')
    }
    return selectedFn.render_raw_curl_for_test(rt, selectedTc.name, ctx, false, false, findMediaFile)
  })

  if (isLoading) {
    return <Loader />
  }

  if (error) {
    return <ErrorMessage error={error instanceof Error ? error.message : 'Unknown error'} />
  }

  if (curl === undefined) {
    return null
  }

  return (
    <WithCopyButton text={curl}>
      <pre className='w-[100%] whitespace-pre-wrap break-all rounded-lg border bg-muted p-4 font-mono text-xs'>
        {curl}
      </pre>
    </WithCopyButton>
  )
}
