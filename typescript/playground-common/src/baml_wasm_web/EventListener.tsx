'use client'
import 'react18-json-view/src/style.css'
// import * as vscode from 'vscode'

import { atom, useAtom, useAtomValue, useSetAtom } from 'jotai'
import { atomWithStorage } from 'jotai/utils'
import { useEffect } from 'react'
import CustomErrorBoundary from '../utils/ErrorFallback'
import { vscodeLocalStorageStore } from './JotaiProvider'
import { vscode } from '@/shared/baml-project-panel/vscode'
import { filesAtom, wasmAtom } from '@/shared/baml-project-panel/atoms'
import {
  selectedFunctionAtom,
  selectedTestcaseAtom,
  updateCursorAtom,
} from '@/shared/baml-project-panel/playground-panel/atoms'
import { useRunTests } from '@/shared/baml-project-panel/playground-panel/prompt-preview/test-panel/test-runner'
import { orchIndexAtom } from '@/shared/baml-project-panel/playground-panel/atoms-orch-graph'

export const hasClosedEnvVarsDialogAtom = atomWithStorage<boolean>(
  'has-closed-env-vars-dialog',
  false,
  vscodeLocalStorageStore,
)
export const bamlCliVersionAtom = atom<string | null>(null)

export const showIntroToChecksDialogAtom = atom(false)
export const hasClosedIntroToChecksDialogAtom = atomWithStorage<boolean>(
  'has-closed-intro-to-checks-dialog',
  false,
  vscodeLocalStorageStore,
)

// const selectedProjectAtom = atom(
//   (get) => {
//     const allProjects = get(availableProjectsAtom)
//     const project = get(selectedProjectStorageAtom)
//     const match = allProjects.find((p) => p === project) ?? allProjects.at(0) ?? null
//     return match
//   },
//   (get, set, project: string) => {
//     if (project !== null) {
//       set(selectedProjectStorageAtom, project)
//     }
//   },
// )

// export const selectedFunctionAtom = atom(
//   (get) => {
//     const functions = get(availableFunctionsAtom)
//     const func = get(selectedFunctionStorageAtom)
//     const match = functions.find((f) => f.name === func) ?? functions.at(0)
//     return match ?? null
//   },
//   (get, set, func: string) => {
//     if (func !== null) {
//       const functions = get(availableFunctionsAtom)
//       if (functions.find((f) => f.name === func)) {
//         set(selectedFunctionStorageAtom, func)
//         set(orchIndexAtom, 0)
//       }
//     }
//   },
// )

// const rawSelectedTestCaseAtom = atom<string | null>(null)
// export const selectedTestCaseAtom = atom(
//   (get) => {
//     const func = get(selectedFunctionAtom)
//     const testCases = func?.test_cases ?? []
//     const testCase = get(rawSelectedTestCaseAtom)
//     const match = testCases.find((tc) => tc.name === testCase) ?? testCases.at(0)
//     return match ?? null
//   },
//   (get, set, testCase: string) => {
//     set(rawSelectedTestCaseAtom, testCase)
//     set(orchIndexAtom, 0)
//   },
// )

// const removeProjectAtom = atom(null, (get, set, root_path: string) => {
//   set(projectFilesAtom(root_path), {})
//   set(projectFamilyAtom(root_path), null)
//   set(runtimeFamilyAtom(root_path), {})
//   const availableProjects = get(availableProjectsAtom)
//   set(
//     availableProjectsAtom,
//     availableProjects.filter((p) => p !== root_path),
//   )
// })

// type WriteFileParams = {
//   reason: string
//   root_path: string
//   files: { name: string; content: string | undefined }[]
// } & (
//   | {
//       replace_all?: true
//     }
//   | {
//       renames?: { from: string; to: string }[]
//     }
// )

// export const updateFileAtom = atom(null, (get, set, params: WriteFileParams) => {
//   const { reason, root_path, files } = params
//   const replace_all = 'replace_all' in params
//   const renames = 'renames' in params ? (params.renames ?? []) : []
//   console.debug(
//     `updateFile: Updating files due to ${reason}: ${files.length} files (${replace_all ? 'replace all' : 'update'})`,
//   )
//   const _projFiles = get(projectFilesAtom(root_path))
//   const filesToDelete = files.filter((f) => f.content === undefined).map((f) => f.name)

//   let projFiles = {
//     ..._projFiles,
//   }
//   const filesToModify = files
//     .filter((f) => f.content !== undefined)
//     .map((f): [string, string] => [f.name, f.content as string])

//   renames.forEach(({ from, to }) => {
//     if (from in projFiles) {
//       projFiles[to] = projFiles[from]
//       delete projFiles[from]
//       filesToDelete.push(from)
//     }
//   })

//   if (replace_all) {
//     for (const file of Object.keys(_projFiles)) {
//       if (!filesToDelete.includes(file)) {
//         filesToDelete.push(file)
//       }
//     }
//     projFiles = Object.fromEntries(filesToModify)
//   }

//   let project = get(projectFamilyAtom(root_path))
//   const wasm = get(wasmAtom)
//   if (project && !replace_all) {
//     for (const file of filesToDelete) {
//       if (file.startsWith(root_path)) {
//         project.update_file(file, undefined)
//       }
//     }
//     console.log('file root path', root_path)
//     for (const [name, content] of filesToModify) {
//       if (name.startsWith(root_path)) {
//         project.update_file(name, content)
//         projFiles[name] = content
//       }
//     }
//   } else {
//     const onlyRelevantFiles = Object.fromEntries(
//       Object.entries(projFiles).filter(([name, _]) => name.startsWith(root_path)),
//     )
//     // console.log('Creating new project', root_path, onlyRelevantFiles)
//     if (wasm) {
//       project = wasm.WasmProject.new(root_path, onlyRelevantFiles)
//     } else {
//       console.log('wasm not yet ready')
//     }
//   }
//   let rt: WasmRuntime | undefined = undefined
//   let diag: WasmDiagnosticError | undefined = undefined

//   if (project && wasm) {
//     try {
//       const envVars = get(envVarsAtom)
//       rt = project.runtime(envVars)
//       diag = project.diagnostics(rt)
//     } catch (e) {
//       const WasmDiagnosticError = wasm.WasmDiagnosticError
//       if (e instanceof Error) {
//         console.error(e.message)
//       } else if (e instanceof WasmDiagnosticError) {
//         diag = e
//       } else {
//         console.error(e)
//       }
//     }
//   }

//   const availableProjects = get(availableProjectsAtom)
//   if (!availableProjects.includes(root_path)) {
//     set(availableProjectsAtom, [...availableProjects, root_path])
//   }

//   set(projectFilesAtom(root_path), projFiles)
//   set(projectFamilyAtom(root_path), project)
//   set(runtimeFamilyAtom(root_path), (prev) => ({
//     last_successful_runtime: prev.current_runtime ?? prev.last_successful_runtime,
//     current_runtime: rt,
//     diagnostics: diag,
//   }))
// })

// export const selectedRuntimeAtom = atom((get) => {
//   const project = get(selectedProjectAtom)
//   if (!project) {
//     return null
//   }

//   const runtime = get(runtimeFamilyAtom(project))
//   if (runtime.current_runtime) return runtime.current_runtime
//   if (runtime.last_successful_runtime) return runtime.last_successful_runtime
//   return null
// })

// export const runtimeRequiredEnvVarsAtom = atom((get) => {
//   const runtime = get(selectedRuntimeAtom)
//   if (!runtime) {
//     return []
//   }

//   return runtime.required_env_vars()
// })

// const selectedDiagnosticsAtom = atom((get) => {
//   const project = get(selectedProjectAtom)
//   if (!project) {
//     return null
//   }

//   const runtime = get(runtimeFamilyAtom(project))
//   return runtime.diagnostics ?? null
// })

// export const versionAtom = atom((get) => {
//   const wasm = get(wasmAtom)

//   if (wasm === undefined) {
//     return 'Loading...'
//   }

//   return wasm.version()
// })

// export const availableClientsAtom = atom<string[]>([])

// export const availableFunctionsAtom = atom((get) => {
//   const runtime = get(selectedRuntimeAtom)
//   if (!runtime) {
//     return []
//   }
//   return runtime.list_functions()
// })

// export const streamCurlAtom = atom(true)
// export const expandImagesAtom = atom(false)

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

// export const rawCurlLoadable = loadable(rawCurlAtomAsync)

// const renderPromptAtomAsync = atom(async (get) => {
//   const wasm = get(wasmAtom)
//   const runtime = get(selectedRuntimeAtom)
//   const func = get(selectedFunctionAtom)
//   const test_case = get(selectedTestCaseAtom)
//   const orch_index = get(orchIndexAtom)
//   if (!wasm || !runtime || !func || !test_case) {
//     return null
//   }

//   const wasmCallContext = new wasm.WasmCallContext()
//   wasmCallContext.node_index = orch_index

//   try {
//     return await func.render_prompt_for_test(runtime, test_case.name, wasmCallContext, async (path: string) => {
//       return await vscode.readFile(path)
//     })
//   } catch (e) {
//     if (e instanceof Error) {
//       return e.message
//     } else {
//       return `${e}`
//     }
//   }
// })

// export const renderPromptAtom = unwrap(renderPromptAtomAsync)

// export const diagnositicsAtom = atom((get) => {
//   const diagnostics = get(selectedDiagnosticsAtom)
//   if (!diagnostics) {
//     return []
//   }

//   return diagnostics.errors()
// })

// export const numErrorsAtom = atom((get) => {
//   const errors = get(diagnositicsAtom)

//   const warningCount = errors.filter((e) => e.type === 'warning').length

//   return { errors: errors.length - warningCount, warnings: warningCount }
// })

// const ErrorCount: React.FC = () => {
//   const { errors, warnings } = useAtomValue(numErrorsAtom)
//   if (errors === 0 && warnings === 0) {
//     return (
//       <div className='flex flex-row gap-1 items-center text-green-600'>
//         <CheckCircle size={12} />
//       </div>
//     )
//   }
//   if (errors === 0) {
//     return (
//       <div className='flex flex-row gap-1 items-center text-yellow-600'>
//         {warnings} <AlertTriangle size={12} />
//       </div>
//     )
//   }
//   return (
//     <div className='flex flex-row gap-1 items-center text-red-600'>
//       {errors} <XCircle size={12} /> {warnings} <AlertTriangle size={12} />{' '}
//     </div>
//   )
// }
// const createRuntime = (
//   wasm: typeof import('@gloo-ai/baml-schema-wasm-web'),
//   envVars: Record<string, string>,
//   root_path: string,
//   project_files: Record<string, string>,
// ) => {
//   const only_project_files = Object.fromEntries(
//     Object.entries(project_files).filter(([name, _]) => name.startsWith(root_path)),
//   )
//   const project = wasm.WasmProject.new(root_path, only_project_files)

//   let rt = undefined
//   let diag = undefined
//   try {
//     rt = project.runtime(envVars)
//     diag = project.diagnostics(rt)
//   } catch (e) {
//     const WasmDiagnosticError = wasm.WasmDiagnosticError
//     if (e instanceof Error) {
//       console.error(e.message)
//     } else if (e instanceof WasmDiagnosticError) {
//       diag = e
//     } else {
//       console.error(e)
//     }
//   }

//   return {
//     project,
//     runtime: rt,
//     diagnostics: diag,
//   }
// }

// We don't use ASTContext.provider because we should the default value of the context
export const EventListener: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const updateCursor = useSetAtom(updateCursorAtom)
  const setFiles = useSetAtom(filesAtom)
  const [selectedFunc, setSelectedFunction] = useAtom(selectedFunctionAtom)
  const setSelectedTestcase = useSetAtom(selectedTestcaseAtom)
  const [bamlCliVersion, setBamlCliVersion] = useAtom(bamlCliVersionAtom)
  const { setRunningTests } = useRunTests()
  const wasm = useAtomValue(wasmAtom)
  useEffect(() => {
    if (wasm) {
      console.log('wasm ready!')
      try {
        vscode.markInitialized()
      } catch (e) {
        console.error('Error marking initialized', e)
      }
    }
  }, [wasm])

  const setOrchestratorIndex = useSetAtom(orchIndexAtom)

  useEffect(() => {
    if (selectedFunc) {
      // todo: maybe we use a derived atom to reset it. But for now this useeffect works.
      setOrchestratorIndex(0)
    }
  }, [selectedFunc])

  useEffect(() => {
    const fn = (
      event: MessageEvent<
        | {
            command: 'modify_file'
            content: {
              root_path: string
              name: string
              content: string | undefined
            }
          }
        | {
            command: 'add_project'
            content: {
              root_path: string
              files: Record<string, string>
            }
          }
        | {
            command: 'remove_project'
            content: {
              root_path: string
            }
          }
        | {
            command: 'select_function'
            content: {
              root_path: string
              function_name: string
            }
          }
        | {
            command: 'update_cursor'
            content: {
              cursor: { fileName: string; fileText: string; line: number; column: number }
            }
          }
        | {
            command: 'port_number'
            content: {
              port: number
            }
          }
        | {
            command: 'baml_cli_version'
            content: string
          }
        | {
            command: 'run_test'
            content: { test_name: string }
          }
      >,
    ) => {
      const { command, content } = event.data
      console.log('command', command)

      switch (command) {
        case 'add_project':
          if (content && content.root_path) {
            setFiles(Object.fromEntries(Object.entries(content.files).map(([name, content]) => [name, content])))
          }
          break

        case 'select_function':
          console.log('select_function', content)
          setSelectedFunction(content.function_name)
          break
        case 'update_cursor':
          if ('cursor' in content) {
            updateCursor(content.cursor)
          }
          break
        case 'baml_cli_version':
          setBamlCliVersion(content)
          break

        case 'remove_project':
          setFiles({})
          break

        case 'run_test':
          if (selectedFunc) {
            setSelectedTestcase(content.test_name)
            setRunningTests([{ functionName: selectedFunc, testName: content.test_name }])
          } else {
            console.error('No function selected')
          }
          // run([content.test_name])
          // setShowTests(true)
          // setClientGraph(false)
          break
      }
    }

    window.addEventListener('message', fn)

    return () => window.removeEventListener('message', fn)
    // If we dont add the jotai atom callbacks here like setRunningTests, this will call an old version of the atom (e.g. runTests which may have undefined dependencies).
  }, [selectedFunc, setRunningTests, updateCursor])

  return (
    <>
      <div className="flex absolute right-2 bottom-2 z-50 flex-row gap-2 text-xs bg-transparent">
        <div className="pr-4 whitespace-nowrap">{bamlCliVersion && 'baml-cli ' + bamlCliVersion}</div>
        {<span>VSCode Runtime Version: {bamlCliVersion}</span>}
      </div>
      {/* {selectedProject === null ? (
        availableProjects.length === 0 ? (
          <div>
            No baml projects loaded yet
            <br />
            Open a baml file or wait for the extension to finish loading!
          </div>
        ) : (
          <div>
            <h1>Projects</h1>
            <div>
              {availableProjects.map((root_dir) => (
                <div key={root_dir}>
                  <VSCodeButton onClick={() => setSelectedProject(root_dir)}>{root_dir}</VSCodeButton>
                </div>
              ))}
            </div>
          </div>
        )
      ) : ( */}
      <CustomErrorBoundary message="Error loading project">{children}</CustomErrorBoundary>
      {/* )} */}
    </>
  )
}
