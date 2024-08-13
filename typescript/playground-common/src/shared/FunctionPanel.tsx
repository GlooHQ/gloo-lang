/// Content once a function has been selected.
import { useAppState } from './AppStateContext'
import { useAtom, useAtomValue, useSetAtom } from 'jotai'
import React, { useState } from 'react'
import useSWR from 'swr'

import '@xyflow/react/dist/style.css'
import {
  wasmAtom,
  renderPromptAtom,
  selectedFunctionAtom,
  selectedRuntimeAtom,
  selectedTestCaseAtom,
  orchIndexAtom,
  expandImagesAtom,
  streamCurlAtom,
  rawCurlLoadable,
} from '../baml_wasm_web/EventListener'
import TestResults from '../baml_wasm_web/test_uis/test_result'
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from '../components/ui/resizable'
import { TooltipProvider } from '../components/ui/tooltip'
import { PromptChunk } from './ImplPanel'
import FunctionTestSnippet from './TestSnippet'
import { Copy } from 'lucide-react'
import { Button } from '../components/ui/button'
import { CheckboxHeader } from './CheckboxHeader'
import { Switch } from '../components/ui/switch'
import { vscode } from '../utils/vscode'
import clsx from 'clsx'

const handleCopy = (text: string) => () => {
  navigator.clipboard.writeText(text)
}

const CurlSnippet: React.FC = () => {
  const rawCurl = useAtomValue(rawCurlLoadable)
  const [streamCurl, setStreamCurl] = useAtom(streamCurlAtom)
  const [expandImages, setExpandImages] = useAtom(expandImagesAtom)

  // if (!wasm || !runtime || !func || !test_case) {
  //   return <div>Not yet ready</div>
  // }

  // const wasmCallContext = new wasm.WasmCallContext()
  // wasmCallContext.node_index = orch_index

  // const rawCurl = useSWR(
  //   { swr: 'CurlSnippet', runtime, func, test_case, orch_index, streamCurl, expandImages },
  //   async () => {
  //     return await func.render_raw_curl_for_test(
  //       runtime,
  //       test_case.name,
  //       wasmCallContext,
  //       streamCurl,
  //       expandImages,
  //       async (path: string) => {
  //         return await vscode.readFile(path)
  //       },
  //     )
  //   },
  // )

  return (
    <div>
      <div className='flex items-center justify-end p-2 space-x-2 rounded-md shadow-sm'>
        <label className='flex items-center mr-2 space-x-1'>
          <Switch
            className='data-[state=checked]:bg-vscode-button-background data-[state=unchecked]:bg-vscode-input-background'
            checked={streamCurl}
            onCheckedChange={setStreamCurl}
          />
          <span>Show Stream Request</span>
        </label>
        <label className='flex items-center mr-2 space-x-1'>
          <Switch
            className='data-[state=checked]:bg-vscode-button-background data-[state=unchecked]:bg-vscode-input-background'
            checked={expandImages}
            onCheckedChange={setExpandImages}
          />
          <span>Show fully expanded command</span>
        </label>
        <Button
          onClick={rawCurl.state === 'hasData' && rawCurl.data ? handleCopy(rawCurl.data) : () => {}}
          className='px-3 py-1 text-xs text-white bg-vscode-button-background hover:bg-vscode-button-hoverBackground'
        >
          <Copy size={16} />
        </Button>
      </div>
      {rawCurl.state === 'loading' ? (
        <div>Loading...</div>
      ) : (
        <PromptChunk
          text={(() => {
            switch (rawCurl.state) {
              case 'hasData':
                return rawCurl.data ?? ''
              case 'hasError':
                return `${rawCurl.error}`
            }
          })()}
          type={(() => {
            switch (rawCurl.state) {
              case 'hasData':
                return 'preview'
              case 'hasError':
                return 'error'
            }
          })()}
          client={{
            identifier: {
              end: 0,
              source_file: '',
              start: 0,
              value: 'Curl Request',
            },
            provider: '',
            model: '',
          }}
          showCopy={true}
        />
      )}
    </div>
  )
}

type WasmChatMessagePartMedia =
  | {
      type: 'url'
      url: string
    }
  | {
      type: 'path'
      path: string
    }

const WebviewMedia: React.FC<{ bamlMediaType: 'image' | 'audio'; media: WasmChatMessagePartMedia }> = ({
  bamlMediaType,
  media,
}) => {
  const pathAsUri = useSWR({ swr: 'WebviewMedia', ...media }, async () => {
    switch (media.type) {
      case 'path':
        const uri = await vscode.asWebviewUri('', media.path)
        // Do a manual check to assert that the image exists
        if ((await fetch(uri, { method: 'HEAD' })).status !== 200) {
          throw new Error('file not found')
        }
        return uri
      case 'url':
        return media.url
    }
  })

  if (pathAsUri.error) {
    const error = typeof pathAsUri.error.message == 'string' ? pathAsUri.error.message : JSON.stringify(pathAsUri.error)
    return (
      <div className='bg-vscode-inputValidation-errorBackground rounded-lg px-2 py-1'>
        <div>
          Error loading {bamlMediaType}: {error}
        </div>
        <div>{media.type === 'path' ? media.path.replace('file://', '') : media.url}</div>
      </div>
    )
  }

  if (pathAsUri.isLoading) {
    return <div>Loading {bamlMediaType}...</div>
  }

  const mediaUrl = pathAsUri.data

  return (
    <div className='p-1'>
      {(() => {
        switch (bamlMediaType) {
          case 'image':
            return (
              <a href={mediaUrl} target='_blank' rel='noopener noreferrer'>
                <img src={mediaUrl} className='max-h-[400px] max-w-[400px] object-left-top object-scale-down' />
              </a>
            )
          case 'audio':
            return (
              <audio controls>
                <source src={mediaUrl} />
                Your browser does not support the audio element.
              </audio>
            )
        }
      })()}
    </div>
  )
}

const PromptPreview: React.FC = () => {
  const promptPreview = useAtomValue(renderPromptAtom)
  const { showCurlRequest } = useAppState()

  if (!promptPreview) {
    return (
      <div className='flex flex-col items-center justify-center w-full h-full gap-2'>
        <span className='text-center'>No prompt preview available! Add a test to see it!</span>
        <FunctionTestSnippet />
      </div>
    )
  }

  if (typeof promptPreview === 'string') {
    return (
      <PromptChunk
        text={promptPreview}
        type='error'
        client={{
          identifier: {
            end: 0,
            source_file: '',
            start: 0,
            value: 'Error',
          },
          provider: 'baml-openai-chat',
          model: 'gpt-4',
        }}
      />
    )
  }

  if (showCurlRequest) {
    return <CurlSnippet />
  }

  return (
    <div className='flex flex-col w-full h-full gap-4 px-2'>
      {promptPreview.as_chat()?.map((chat, idx) => (
        <div key={idx} className='flex flex-col'>
          <div className='flex flex-row'>{chat.role}</div>
          {chat.parts.map((part, idx) => {
            if (part.is_text())
              return (
                <PromptChunk
                  key={idx}
                  text={part.as_text()!}
                  client={{
                    identifier: {
                      end: 0,
                      source_file: '',
                      start: 0,
                      value: promptPreview.client_name,
                    },
                    provider: 'baml-openai-chat',
                    model: 'gpt-4',
                  }}
                />
              )
            if (part.is_image()) {
              const media = part.as_media()
              if (!media) return <div>Error loading image: this chat message part is not media</div>
              if (media.type === 'error') return <div>Error loading image: {media.error}</div>
              return <WebviewMedia key={idx} bamlMediaType='image' media={part.as_media()} />
            }
            if (part.is_audio()) {
              const media = part.as_media()
              if (!media) return <div>Error loading audio: this chat message part is not media</div>
              if (media.type === 'error') return <div>Error loading audio: {media.error}</div>
              return <WebviewMedia key={idx} bamlMediaType='audio' media={part.as_media()} />
            }
            return null
          })}
        </div>
      ))}
    </div>
  )
}

const FunctionPanel: React.FC = () => {
  const selectedFunc = useAtomValue(selectedFunctionAtom)
  const { showTestResults } = useAppState()

  if (!selectedFunc) {
    const bamlFunctionSnippet = `
function ClassifyConversation(convo: string[]) -> Topic[] {
  client GPT4
  prompt #"
    Classify the CONVERSATION.

    {{ ctx.output_format }}

    CONVERSATION:
    {% for c in convo %}
    {{ c }}
    {% endfor %}
  "#
}

enum Topic {
  TechnicalSupport
  Sales
  CustomerService
  Other
}
  `.trim()
    return (
      <div className='flex flex-col items-center justify-center w-full h-full gap-2'>
        No functions found! You can create a new function like:
        <pre className='p-2 text-xs rounded-sm bg-vscode-input-background'>{bamlFunctionSnippet}</pre>
      </div>
    )
  }

  return (
    <div
      className='flex flex-col w-full overflow-auto'
      style={{
        height: 'calc(100vh - 80px)',
      }}
    >
      <TooltipProvider>
        <ResizablePanelGroup direction='vertical' className='h-full'>
          <ResizablePanel id='top-panel' className='flex w-full px-1' defaultSize={50}>
            <div className='w-full'>
              <ResizablePanelGroup direction='horizontal' className='h-full pb-4'>
                <div className='w-full h-full'>
                  <CheckboxHeader />
                  <div className='relative w-full overflow-y-auto' style={{ height: 'calc(100% - 32px)' }}>
                    <PromptPreview />
                  </div>
                </div>
              </ResizablePanelGroup>

              {/* </Allotment> */}
            </div>
          </ResizablePanel>
          {showTestResults && (
            <>
              <ResizableHandle withHandle={false} className='bg-vscode-panel-border' />
              <ResizablePanel
                minSize={10}
                className='flex h-full px-0 py-2 pb-3 border-t border-vscode-textSeparator-foreground'
              >
                <TestResults />
              </ResizablePanel>
            </>
          )}
        </ResizablePanelGroup>
      </TooltipProvider>
    </div>
  )
}

export default FunctionPanel
