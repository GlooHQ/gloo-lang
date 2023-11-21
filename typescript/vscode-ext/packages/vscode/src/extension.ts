/* eslint-disable @typescript-eslint/no-var-requires */
/* eslint-disable @typescript-eslint/no-unsafe-assignment */
import * as vscode from 'vscode'

import plugins from './plugins'
import { WebPanelView } from './panels/WebPanelView'
import { BamlDB } from './plugins/language-server'
import testExecutor from './panels/execute_test'
import glooLens from './GlooCodeLensProvider'

const outputChannel = vscode.window.createOutputChannel('baml')
const diagnosticsCollection = vscode.languages.createDiagnosticCollection('baml')
const LANG_NAME = 'Baml'

export function activate(context: vscode.ExtensionContext) {
  const config = vscode.workspace.getConfiguration('baml')
  testExecutor.start()

  const bamlPlygroundCommand = vscode.commands.registerCommand('baml.openBamlPanel', (args) => {
    const initialFunctionName = args.functionName
    const initialImplName = args.implName
    const config = vscode.workspace.getConfiguration()
    config.update('baml.bamlPanelOpen', true, vscode.ConfigurationTarget.Global)
    console.log('Opening BAML Panel', initialFunctionName, initialImplName)
    WebPanelView.render(context.extensionUri)

    WebPanelView.currentPanel?.postMessage('setDb', Array.from(BamlDB.entries()))
    console.log("setresource");
    WebPanelView.currentPanel?.postMessage('setSelectedResource', {
      functionName: initialFunctionName,
      implName: initialImplName,
    })
  })

  context.subscriptions.push(bamlPlygroundCommand)
  context.subscriptions.push(vscode.languages.registerCodeLensProvider(
    { scheme: 'file', language: "baml" },
    glooLens
  ))


  plugins.map(async (plugin) => {
    const enabled = await plugin.enabled()
    if (enabled) {
      console.log(`Activating ${plugin.name}`)
      if (plugin.activate) {
        await plugin.activate(context, outputChannel)
      }
    } else {
      console.log(`${plugin.name} is Disabled`)
    }
  })
}

export function deactivate(): void {
  testExecutor.close()
  console.log("deactivate");
  plugins.forEach((plugin) => {
    if (plugin.deactivate) {
      void plugin.deactivate()
    }
  })
}
