import { atom, useAtomValue } from 'jotai'
import { atomFamily, atomWithStorage, createJSONStorage } from 'jotai/utils'

import { unwrap } from 'jotai/utils'
import { type ICodeBlock } from './types'
import { vscodeLocalStorageStore } from './Jotai'
import { orchIndexAtom } from './playground-panel/atoms-orch-graph'
import { vscode } from './vscode'

const wasmAtomAsync = atom(async () => {
  const wasm = await import('@gloo-ai/baml-schema-wasm-web/baml_schema_build')
  return wasm
})

export const wasmAtom = unwrap(wasmAtomAsync)

export const useWaitForWasm = () => {
  const wasm = useAtomValue(wasmAtom)
  return wasm !== undefined
}

export const filesAtom = atom<Record<string, string>>({})
export const sandboxFilesAtom = atom<Record<string, string>>({})

const pythonGenerator = `
generator python {
    // Valid values: "python/pydantic", "typescript", "ruby/sorbet"
    output_type "python/pydantic"
    
    // Where the generated code will be saved (relative to baml_src/)
    output_dir "python"
    
    // What interface you prefer to use for the generated code (sync/async)
    // Both are generated regardless of the choice, just modifies what is exported
    // at the top level
    default_client_mode "sync"
    
    // Version of runtime to generate code for (should match installed baml-py version)
    version "0.66.0"
}

generator typescript {
    // Valid values: "python/pydantic", "typescript", "ruby/sorbet"
    output_type "typescript"
    
    // Where the generated code will be saved (relative to baml_src/)
    output_dir "typescript"
    
    // Version of runtime to generate code for (should match installed baml-py version)
    version "0.66.0"
}

`
export const projectAtom = atom((get) => {
  const wasm = get(wasmAtom)
  const files = get(filesAtom)
  if (wasm === undefined) {
    return undefined
  }
  // filter out files that are not baml files
  const bamlFiles = Object.entries(files).filter(([path, content]) => path.endsWith('.baml'))
  // TODO: add python generator if using sandbox
  // files = files + pythonGenerator

  return wasm.WasmProject.new('./', bamlFiles)
})

export const ctxAtom = atom((get) => {
  const wasm = get(wasmAtom)
  if (wasm === undefined) {
    return undefined
  }
  const context = new wasm.WasmCallContext()
  const orch_index = get(orchIndexAtom)
  context.node_index = orch_index
  return context
})

export const runtimeAtom = atom((get) => {
  try {
    const wasm = get(wasmAtom)
    const project = get(projectAtom)
    const envVars = get(envVarsAtom)
    if (wasm === undefined || project === undefined) {
      return { rt: undefined, diags: undefined }
    }
    const selectedEnvVars = Object.fromEntries(Object.entries(envVars).filter(([key, value]) => value !== undefined))
    const rt = project.runtime(selectedEnvVars)
    const diags = project.diagnostics(rt)
    return { rt, diags }
  } catch (e) {
    console.log('Error occurred while getting runtime', e)
    const wasm = get(wasmAtom)
    if (wasm) {
      const WasmDiagnosticError = wasm.WasmDiagnosticError
      if (e instanceof WasmDiagnosticError) {
        return { rt: undefined, diags: e }
      }
    }
    if (e instanceof Error) {
      console.error(e.message)
    } else {
      console.error(e)
    }
  }
  return { rt: undefined, diags: undefined }
})

export const diagnosticsAtom = atom((get) => {
  const runtime = get(runtimeAtom)
  return runtime.diags?.errors() ?? []
})

export const numErrorsAtom = atom((get) => {
  const errors = get(diagnosticsAtom)

  const warningCount = errors.filter((e) => e.type === 'warning').length

  return { errors: errors.length - warningCount, warnings: warningCount }
})

// todo debounce this.
export const generatedFilesAtom = atom((get) => {
  const project = get(projectAtom)
  if (project === undefined) {
    return undefined
  }
  const runtime = get(runtimeAtom)
  if (runtime.rt === undefined) {
    return undefined
  }

  const generators = project.run_generators()
  const files = generators.flatMap((gen) =>
    gen.files.map((f) => ({
      path: f.path_in_output_dir,
      content: f.contents,
      outputDir: gen.output_dir,
    })),
  )
  return files
})

export const generatedFilesByLangAtom = atomFamily((lang: ICodeBlock['language']) =>
  atom((get) => {
    const allFiles = get(generatedFilesAtom)
    if (!allFiles) return undefined

    return allFiles
      .filter((f) => f.outputDir.includes(lang))
      .map(({ path, content }) => ({
        path,
        content,
      }))
  }),
)

export const isPanelVisibleAtom = atom(false)

const vscodeSettingsAtom = unwrap(
  atom(async () => {
    try {
      const res = await vscode.getIsProxyEnabled()
      return {
        enablePlaygroundProxy: res,
      }
    } catch (e) {
      console.error(`Error occurred while getting vscode settings:\n${JSON.stringify(e)}`)
      return {
        enablePlaygroundProxy: true,
      }
    }
  }),
)

const playgroundPortAtom = unwrap(
  atom(async () => {
    try {
      const res = await vscode.getPlaygroundPort()
      return res
    } catch (e) {
      console.error(`Error occurred while getting playground port:\n${JSON.stringify(e)}`)
      return 0
    }
  }),
)

export const resetEnvKeyValuesAtom = atom(null, (get, set) => {
  set(envKeyValueStorage, [])
})
export const envKeyValuesAtom = atom(
  (get) => {
    return get(envKeyValueStorage).map(([k, v], idx): [string, string, number] => [k, v, idx])
  },
  (
    get,
    set,
    update: // Update value
      | { itemIndex: number; value: string }
      // Update key
      | { itemIndex: number; newKey: string }
      // Remove key
      | { itemIndex: number; remove: true }
      // Insert key
      | {
          itemIndex: null
          key: string
          value?: string
        },
  ) => {
    if (update.itemIndex !== null) {
      const keyValues = [...get(envKeyValueStorage)]
      if ('value' in update) {
        keyValues[update.itemIndex][1] = update.value
      } else if ('newKey' in update) {
        keyValues[update.itemIndex][0] = update.newKey
      } else if ('remove' in update) {
        keyValues.splice(update.itemIndex, 1)
      }
      console.log('Setting env key values', keyValues)
      set(envKeyValueStorage, keyValues)
    } else {
      set(envKeyValueStorage, (prev) => [...prev, [update.key, update.value ?? '']])
    }
  },
)
export const envVarsAtom = atom(
  (get) => {
    if ((window as any).next?.version) {
      // NextJS environment doesnt have vscode settings, and proxy is always enabled
      return Object.fromEntries(defaultEnvKeyValues.map(([k, v]) => [k, v]))
    } else {
      const vscodeSettings = get(vscodeSettingsAtom)
      console.log('vscodeSettings', vscodeSettings)
      if (vscodeSettings?.enablePlaygroundProxy !== undefined && !vscodeSettings?.enablePlaygroundProxy) {
        // filter it out
        const envKeyValues = get(envKeyValuesAtom)
        return Object.fromEntries(envKeyValues.map(([k, v]) => [k, v]).filter(([k]) => k !== 'BOUNDARY_PROXY_URL'))
      }

      const envKeyValues = get(envKeyValuesAtom)
      const port = get(playgroundPortAtom)
      const entries = envKeyValues.map(([k, v]) => {
        if (k === 'BOUNDARY_PROXY_URL' && port !== 0) {
          return [k, `http://localhost:${port}`]
        }
        return [k, v]
      })
      return Object.fromEntries(entries)
    }
  },
  (get, set, newEnvVars: Record<string, string>) => {
    const envKeyValues = Object.entries(newEnvVars)
    set(envKeyValueStorage, envKeyValues)
  },
)

export const requiredEnvVarsAtom = atom((get) => {
  const { rt } = get(runtimeAtom)
  if (rt === undefined) {
    return []
  }
  const requiredEnvVars = rt.required_env_vars()

  const defaultEnvVars = ['OPENAI_API_KEY', 'ANTHROPIC_API_KEY']
  defaultEnvVars.forEach((e) => {
    if (!requiredEnvVars.find((envVar) => e === envVar)) {
      requiredEnvVars.push(e)
    }
  })

  return requiredEnvVars
})

const defaultEnvKeyValues: [string, string][] = (() => {
  if ((window as any).next?.version) {
    console.log('Running in nextjs')

    const domain = window?.location?.origin || ''
    if (domain.includes('localhost')) {
      // we can do somehting fancier here later if we want to test locally.
      return [['BOUNDARY_PROXY_URL', 'https://fiddle-proxy.fly.dev']]
    }
    return [['BOUNDARY_PROXY_URL', 'https://fiddle-proxy.fly.dev']]
  } else {
    console.log('Not running in a Next.js environment, set default value')
    // Not running in a Next.js environment, set default value
    return [['BOUNDARY_PROXY_URL', 'http://localhost:0000']]
  }
})()
export const envKeyValueStorage = atomWithStorage<[string, string][]>(
  'env-key-values',
  defaultEnvKeyValues,
  vscodeLocalStorageStore,
)
