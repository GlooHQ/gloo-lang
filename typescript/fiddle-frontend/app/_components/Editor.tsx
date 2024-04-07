'use client'

import CodeMirror, { EditorView, useCodeMirror } from '@uiw/react-codemirror'
import { rust } from '@codemirror/lang-rust'
import { BAML } from '@baml/codemirror-lang'
import { vscodeDark } from '@uiw/codemirror-theme-vscode'
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from '@/components/ui/resizable'
import parser from '@baml/codemirror-lang'
import { useEffect, useRef, useState } from 'react'
import { ASTProvider, FunctionSelector, FunctionPanel, CustomErrorBoundary } from '@baml/playground-common'
import { completeFromList } from '@codemirror/autocomplete'
import { Button } from '@/components/ui/button'
// import '../colors.css'

// import { LRLanguage, LanguageSupport } from '@codemirror/language'

// let parserWithMetadata = parser.configure({
//   props: [
//     styleTags({
//       Identifier: t.variableName,
//       Boolean: t.bool,
//       String: t.string,
//       LineComment: t.lineComment,
//       '( )': t.paren,
//     }),
//     indentNodeProp.add({
//       Application: (context) => context.column(context.node.from) + context.unit,
//     }),
//     foldNodeProp.add({
//       Application: foldInside,
//     }),
//   ],
// })
// const bamlLanguage = LRLanguage.define({
//   parser: parserWithMetadata,
//   languageData: {
//     commentTokens: { line: '#' },
//   },
// })

// export const exampleCompletion = exampleLanguage.data.of({
//   autocomplete: completeFromList([
//     { label: 'defun', type: 'keyword' },
//     { label: 'defvar', type: 'keyword' },
//     { label: 'let', type: 'keyword' },
//     { label: 'cons', type: 'function' },
//     { label: 'car', type: 'function' },
//     { label: 'cdr', type: 'function' },
//   ]),
// })
// return new LanguageSupport(exampleLanguage, [exampleCompletion])

const extensions = [BAML(), EditorView.lineWrapping]
const defaultMainBaml = `
generator lang_python {
  language python
  // This is where your non-baml source code located
  // (relative directory where pyproject.toml, package.json, etc. lives)
  project_root ".."
  // This command is used by "baml test" to run tests
  // defined in the playground
  test_command "pytest -s"
  // This command is used by "baml update-client" to install
  // dependencies to your language environment
  install_command "poetry add baml@latest"
  package_version_command "poetry show baml"
}

function ExtractVerbs {
    input string
    /// list of verbs
    output string[]
}

client<llm> GPT4 {
  provider baml-openai-chat
  options {
    model gpt-4 
    api_key env.OPENAI_API_KEY
  }
}

impl<llm, ExtractVerbs> version1 {
  client GPT4
  prompt #"
    Extract the verbs from this INPUT:
 
    INPUT:
    ---
    {#input}
    ---
    {// this is a comment inside a prompt! //}
    Return a {#print_type(output)}.

    Response:
  "#
}

`

export const Editor = () => {
  const [value, setValue] = useState(defaultMainBaml)

  useEffect(() => {
    const handleKeyDown = (event: any) => {
      // Check if either Ctrl+S or Command+S is pressed
      if ((event.ctrlKey || event.metaKey) && (event.key === 's' || event.keyCode === 83)) {
        event.preventDefault()
        // Place your custom save logic here
        console.log('Custom save action triggered')
      }
    }

    // Add the event listener
    window.addEventListener('keydown', handleKeyDown)

    // Remove the event listener on cleanup
    return () => {
      window.removeEventListener('keydown', handleKeyDown)
    }
  }, [])

  useEffect(() => {
    const lintWithWasm = async () => {
      const lint = await import('@gloo-ai/baml-schema-wasm-web').then((m) => m.lint)
      const linterInput: LinterInput = {
        root_path: 'project/baml_src',
        files: [
          {
            path: 'path/main.baml',
            content: value,
          },
        ],
      }
      console.info(`Linting ${linterInput.files.length} files in ${linterInput.root_path}`)
      const res = lint(JSON.stringify(linterInput))
      const parsedRes = JSON.parse(res)
      console.log(`res ${JSON.stringify(res, null, 2)}`)
      const BamlDB = new Map<string, any>()
      // res is of type ParserDB
      BamlDB.set('baml_src', res)

      if (parsedRes.ok) {
        window.postMessage({
          command: 'setDb',
          content: [['project/baml_src', parsedRes.response]],
        })
      }
    }
    lintWithWasm()
  }, [value])

  return (
    <>
      <ResizablePanelGroup className="min-h-[200px] w-full rounded-lg border overflow-clip" direction="horizontal">
        <ResizablePanel defaultSize={50}>
          <div className="flex w-full h-full">
            <CodeMirror
              value={value}
              extensions={extensions}
              theme={vscodeDark}
              height="100%"
              width="100%"
              maxWidth="100%"
              style={{ width: '100%', height: '100%' }}
              onChange={async (val, viewUpdate) => {
                setValue(val)
              }}
            />
          </div>
        </ResizablePanel>
        <ResizableHandle withHandle />

        <ResizablePanel defaultSize={50}>
          <div className="flex flex-row h-full bg-vscode-panel-background">
            <PlaygroundView />
          </div>
        </ResizablePanel>
      </ResizablePanelGroup>
    </>
  )
}

type LintResponse = {
  diagnostics: LinterError[]
} & (
  | { ok: false }
  | {
      ok: true
      response: any
    }
)

export interface LinterError {
  start: number
  end: number
  text: string
  is_warning: boolean
  source_file: string
}

export interface LinterSourceFile {
  path: string
  content: string
}

export interface LinterInput {
  root_path: string
  files: LinterSourceFile[]
}

const PlaygroundView = () => {
  return (
    <>
      <CustomErrorBoundary>
        <ASTProvider>
          <div className="absolute z-10 flex flex-col items-end gap-1 right-1 top-2 text-end">
            {/* <TestToggle /> */}
            {/* <VSCodeLink href="https://docs.boundaryml.com">Docs</VSCodeLink> */}
          </div>
          <div className="flex flex-col gap-2 px-2 pb-4">
            <FunctionSelector />
            {/* <Separator className="bg-vscode-textSeparator-foreground" /> */}
            <FunctionPanel />
          </div>
        </ASTProvider>
      </CustomErrorBoundary>
    </>
  )
}

export default Editor
