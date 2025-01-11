import JsonView from '@uiw/react-json-view'
import { githubLightTheme as lightTheme } from '@uiw/react-json-view/githubLight'
import { vscodeTheme as darkTheme } from '@uiw/react-json-view/vscode'
import { type WasmFunctionResponse, type WasmTestResponse } from '@gloo-ai/baml-schema-wasm-web'
import { useTheme } from 'next-themes'
import { RenderPromptPart } from '../../render-text'
import { ScrollArea } from '@/components/ui/scroll-area'

// Renders the parsed response only
export const ParsedResponseRenderer: React.FC<{
  response?: WasmFunctionResponse | WasmTestResponse
}> = ({ response }) => {
  if (!response || !response.parsed_response()) {
    return <div className="text-xs text-muted-foreground">Waiting for response...</div>
  }

  const parsedResponse = response.parsed_response()
  if (parsedResponse === undefined) {
    return null
  }
  if (typeof parsedResponse === 'string') {
    return <ParsedResponseRender response={parsedResponse} />
  }

  return <ParsedResponseRender response={parsedResponse.value} />
}

const ParsedResponseRender = ({ response }: { response: string }) => {
  const { theme } = useTheme()

  let parsedResponseObj
  try {
    parsedResponseObj = JSON.parse(response ?? '{}')
  } catch (e) {
    parsedResponseObj = response
  }

  if (typeof parsedResponseObj === 'string') {
    return <RenderPromptPart text={parsedResponseObj} />
  }

  return (
    <div className="flex max-h-[500px]  text-xs">
      <ScrollArea className="pr-2 w-full text-xs" type="always">
        <JsonView
          className="p-1 w-full rounded-md"
          value={parsedResponseObj}
          collapsed={false}
          enableClipboard={true}
          displayDataTypes={false}
          displayObjectSize={true}
          indentWidth={16}
          shortenTextAfterLength={700}
          style={theme === 'dark' ? darkTheme : lightTheme}
        >
          <JsonView.String
            render={({ children, ...reset }, { type, value, keyName }) => {
              if (type === 'type') {
                return <span />
              }
              if (type === 'value') {
                return (
                  <span {...reset} className="whitespace-pre-wrap break-all">
                    &quot;{children}&quot;<span className="text-muted-foreground">, </span>
                  </span>
                )
              }
            }}
          />
          <JsonView.Colon
            render={(props, { parentValue, value, keyName }) => {
              if (Array.isArray(parentValue) && props.children == ':') {
                return <span />
              }
              return <span {...props}>: </span>
            }}
          />

          <JsonView.Null
            render={({ children, ...reset }) => (
              <span {...reset} className="whitespace-pre-wrap break-words">
                null
              </span>
            )}
          />
          <JsonView.Undefined
            render={({ children, ...reset }) => (
              <span {...reset} className="whitespace-pre-wrap break-words">
                undefined
              </span>
            )}
          />
          <JsonView.KeyName
            render={({ ...props }, { parentValue, value, keyName }) => {
              if (Array.isArray(parentValue) && Number.isFinite(props.children)) {
                return <span className="" />
              }
              return <span className="" {...props} />
            }}
          />
        </JsonView>
      </ScrollArea>
    </div>
  )
}
