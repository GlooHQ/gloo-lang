/* eslint-disable @typescript-eslint/no-misused-promises */
import * as vscode from 'vscode'
import axios from 'axios'
import glooLens from './LanguageToBamlCodeLensProvider'
import { WebPanelView, openPlaygroundConfig } from './panels/WebPanelView'
import plugins from './plugins'
import { requestBamlCLIVersion, requestDiagnostics } from './plugins/language-server'
import { telemetry } from './plugins/language-server'
import cors from 'cors'
import { createProxyMiddleware } from 'http-proxy-middleware'
import { type LanguageClient, type ServerOptions, TransportKind } from 'vscode-languageclient/node'
import http2 from "node:http2";

let client: LanguageClient

const outputChannel = vscode.window.createOutputChannel('baml')
const diagnosticsCollection = vscode.languages.createDiagnosticCollection('baml-diagnostics')
const LANG_NAME = 'Baml'

let timeout: NodeJS.Timeout | undefined
let statusBarItem: vscode.StatusBarItem
let server: any

function scheduleDiagnostics(): void {
  if (timeout) {
    clearTimeout(timeout)
  }
  timeout = setTimeout(() => {
    statusBarItem.show()
    runDiagnostics()
  }, 1000) // 1 second after the last keystroke
}

interface LintRequest {
  lintingRules: string[]
  promptTemplate: string
  promptVariables: { [key: string]: string }
}

interface LinterOutput {
  exactPhrase: string
  reason: string
  severity: string
  recommendation?: string
  recommendation_reason?: string
  fix?: string
}

interface LinterRuleOutput {
  diagnostics: LinterOutput[]
  ruleName: string
}

async function runDiagnostics(): Promise<void> {
  const editor = vscode.window.activeTextEditor
  if (!editor) {
    statusBarItem.hide()
    return
  }

  console.log('Running diagnostics')

  statusBarItem.text = `$(sync~spin) Running AI Linter...`
  statusBarItem.backgroundColor = '##9333ea'
  statusBarItem.color = '#ffffff'
  const text = editor.document.getText()

  const lintRequest: LintRequest = {
    lintingRules: ['Rule1', 'Rule2'],
    promptTemplate: text,
    promptVariables: {},
  }
  const diagnostics: vscode.Diagnostic[] = []

  try {
    const response = await axios.post<LinterRuleOutput[]>('http://localhost:8000/lint', lintRequest)
    console.log('Got response:', response.data)
    const results = response.data

    results.forEach((rule) => {
      let found = false

      rule.diagnostics.forEach((output) => {
        const escapedPhrase = output.exactPhrase.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
        const phrase = output.exactPhrase
        let index = 0
        // Find all occurrences of the phrase
        while ((index = text.indexOf(phrase, index)) !== -1) {
          found = true
          const startPos = editor.document.positionAt(index)
          const endPos = editor.document.positionAt(index + phrase.length)
          const range = new vscode.Range(startPos, endPos)

          const diagnostic = new vscode.Diagnostic(
            range,
            `${output.reason}${output.recommendation ? ` - ${output.recommendation}` : ''}`,
            output.severity === 'error' ? vscode.DiagnosticSeverity.Error : vscode.DiagnosticSeverity.Warning,
          )

          if (output.fix) {
            diagnostic.code = '[linter]' + output.fix
          }
          diagnostic.source = rule.ruleName

          diagnostics.push(diagnostic)
          index += phrase.length // Move index to the end of the current found phrase to continue searching
        }

        if (!found && phrase.length > 100) {
          const subPhrase = phrase.substring(0, 100)
          index = 0 // Reset index for new search
          while ((index = text.indexOf(subPhrase, index)) !== -1) {
            const startPos = editor.document.positionAt(index)
            const endPos = editor.document.positionAt(index + subPhrase.length)
            const range = new vscode.Range(startPos, endPos)

            const diagnostic = new vscode.Diagnostic(
              range,
              `${output.reason}${output.recommendation ? ` - ${output.recommendation}` : ''}`,
              output.severity === 'error' ? vscode.DiagnosticSeverity.Error : vscode.DiagnosticSeverity.Warning,
            )

            if (output.fix) {
              diagnostic.code = '[linter]' + output.fix
            }
            diagnostic.source = rule.ruleName

            diagnostics.push(diagnostic)
            index += subPhrase.length // Move index to the end of the current found phrase to continue searching
          }
        }

        // const newRegex = new RegExp(`\\b${}\\b`, 'gi');
      })
    })
    console.log('Pushing test errorrrr')

    console.log('Diagnostics:', diagnostics)
    diagnosticsCollection.clear()
    diagnosticsCollection.set(editor.document.uri, diagnostics)
  } catch (error) {
    console.error('Failed to run diagnostics:', error)
    vscode.window.showErrorMessage('Failed to run diagnostics')
  }
  statusBarItem.text = 'AI Linter Ready'
  statusBarItem.hide()
}

import type { Express } from 'express'
import StatusBarPanel from './panels/StatusBarPanel'

export function activate(context: vscode.ExtensionContext) {
  console.log('BAML extension activating')

  vscode.workspace.getConfiguration('baml')
  // TODO: Reactivate linter.
  // statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100)
  // statusBarItem.text = `AI Linter Ready`
  // statusBarItem.show()
  context.subscriptions.push(StatusBarPanel.instance)

  const provider = new DiagnosticCodeActionProvider()
  const selector: vscode.DocumentSelector = { scheme: 'file', language: 'baml' } // Adjust language as necessary
  const codeActionProvider = vscode.languages.registerCodeActionsProvider(selector, provider, {
    providedCodeActionKinds: [vscode.CodeActionKind.QuickFix],
  })

  context.subscriptions.push(codeActionProvider)

  const app: Express = require('express')()
  app.use(cors())
  let port: number
  const server = app.listen(0, () => {
    console.log('Server started on port ' + getPort())
    WebPanelView.currentPanel?.postMessage('port_number', {
      port: port,
    })
  })

  const getPort = () => {
    const addr = server.address()
    if (addr === null) {
      vscode.window.showErrorMessage(
        'Failed to start BAML extension server. Please try reloading the window, or restarting VSCode.',
      )
      console.error('Failed to start BAML extension server. Please try reloading the window, or restarting VSCode.')
      return 0
    }
    if (typeof addr === 'string') {
      return parseInt(addr)
    }
    return addr.port
  }

  app.use(async (req, res) => {
    const originalUrl = req.headers['baml-original-url'];
    if (typeof originalUrl !== 'string') {
      console.log('baml-original-url header is missing or invalid')
      throw new Error('baml-original-url header is missing or invalid')
    }

    try {
      // Parse the original URL and append the request path.
      const targetUrl = new URL(originalUrl);

      let pathRewrite = req.path;

      // Remove the path in the case of images. Since we request things differently for image GET requests, where we add the url to localhost:4500/actual-url.png
      // to prevent caching issues with Rust reqwest.
      // But for normal completion POST requests, we always call localhost:4500.
      // The original url is always in baml-original-url header.

      // Check for file extensions and set path to empty string.
      if (/\.[a-zA-Z0-9]+$/.test(req.path) && req.method === 'GET') {
        pathRewrite = '';
      }

      // Remove trailing slash
      if (req.path.endsWith('/')) {
        pathRewrite = req.path.slice(0, -1)
      }

      targetUrl.pathname = `${targetUrl.pathname}${pathRewrite}`;

      const proxyReqHeaders = { ...req.headers }; // Clone incoming headers
      delete proxyReqHeaders.host; // Remove host header for upstream requests
      delete proxyReqHeaders.origin; // Remove origin header for upstream requests
      delete req.headers['baml-original-url']; // Remove the custom header

      // Establish HTTP/2 connection
      const client = http2.connect(targetUrl.origin);

      const proxyReq = client.request({
        ':method': req.method,
        ':path': `${targetUrl.pathname}${targetUrl.search}`,
        ...proxyReqHeaders,
      });

      // Pipe the request body to the upstream server.
      req.pipe(proxyReq);

      // Handle the response from the upstream server.
      proxyReq.on('response', (headers) => {
        // Set response headers
        for (const [key, value] of Object.entries(headers)) {
          if (key.startsWith(':')) continue; // Skip pseudo-headers
          res.setHeader(key, value as any);
        }
        res.setHeader('Access-Control-Allow-Origin', '*');
        res.statusCode = headers[':status'] as number;
      });

      proxyReq.on('data', (chunk) => {
        res.write(chunk); // Forward the data to the client
      });

      proxyReq.on('end', () => {
        res.end(); // End the response
        client.close(); // Close the HTTP/2 connection
      });

      proxyReq.on('error', (err) => {
        console.error('Proxy request error:', err);
        res.status(500).send('Internal Server Error');
        client.close();
      });
    } catch (err) {
      console.error('Proxy error:', err);
      res.status(500).send('Failed to process request');
    }
  });

  const bamlPlaygroundCommand = vscode.commands.registerCommand(
    'baml.openBamlPanel',
    (args?: { projectId?: string; functionName?: string; implName?: string; showTests?: boolean }) => {
      const config = vscode.workspace.getConfiguration()
      config.update('baml.bamlPanelOpen', true, vscode.ConfigurationTarget.Global)

      WebPanelView.render(context.extensionUri, getPort, telemetry)
      if (telemetry) {
        telemetry.sendTelemetryEvent({
          event: 'baml.openBamlPanel',
          properties: {},
        })
      }
      // sends project files as well to webview
      requestDiagnostics()

      openPlaygroundConfig.lastOpenedFunction = args?.functionName ?? 'default'
      WebPanelView.currentPanel?.postMessage('select_function', {
        root_path: 'default',
        function_name: args?.functionName ?? 'default',
      })

      console.info('Opening BAML panel')
    },
  )

  const bamlTestcaseCommand = vscode.commands.registerCommand(
    'baml.runBamlTest',
    (args?: {
      projectId: string
      functionName?: string
      implName?: string
      showTests?: boolean
      testCaseName?: string
    }) => {
      WebPanelView.render(context.extensionUri, getPort, telemetry)
      if (telemetry) {
        telemetry.sendTelemetryEvent({
          event: 'baml.runBamlTest',
          properties: {},
        })
      }

      // sends project files as well to webview
      requestDiagnostics()

      openPlaygroundConfig.lastOpenedFunction = args?.functionName ?? 'default'
      WebPanelView.currentPanel?.postMessage('select_function', {
        root_path: 'default',
        function_name: args?.functionName ?? 'default',
      })

      WebPanelView.currentPanel?.postMessage('run_test', {
        test_name: args?.testCaseName ?? 'default',
      })

      console.info('Opening BAML panel')
    },
  )

  context.subscriptions.push(bamlPlaygroundCommand)
  console.log('pushing glooLens')

  const pythonSelector = { language: 'python', scheme: 'file' }
  const typescriptSelector = { language: 'typescript', scheme: 'file' }

  context.subscriptions.push(
    vscode.languages.registerCodeLensProvider(pythonSelector, glooLens),
    vscode.languages.registerCodeLensProvider(typescriptSelector, glooLens),
  )

  context.subscriptions.push(diagnosticsCollection)

  vscode.window.onDidChangeActiveTextEditor((event) => {
    // makes it so we reload the project. Could probably be called reloadProjectFiles or something. This is because we may be clicking into a different file in a separate baml_src.
    requestDiagnostics()
  })

  // Add cursor movement listener
  vscode.window.onDidChangeTextEditorSelection((event) => {
    const position = event.selections[0].active

    const editor = vscode.window.activeTextEditor

    if (editor) {
      const name = editor.document.fileName
      const text = editor.document.getText()

      // TODO: buggy when used with multiple functions, needs a fix.
      WebPanelView.currentPanel?.postMessage('update_cursor', {
        cursor: {
          fileName: name,
          fileText: text,
          line: position.line + 1,
          column: position.character,
        },
      })
    }
  })

  const config = vscode.workspace.getConfiguration('editor', { languageId: 'baml' })
  if (!config.get('defaultFormatter')) {
    // TODO: once the BAML formatter is stable, we should auto-prompt people to set it as the default formatter.
    // void vscode.commands.executeCommand('baml.setDefaultFormatter')
  }

  // Listen for messages from the webview

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

  if (process.env.VSCODE_DEBUG_MODE === 'true') {
    console.log(`vscode env: ${JSON.stringify(process.env, null, 2)}`)
    vscode.commands.executeCommand('baml.openBamlPanel')
  }

  setInterval(() => {
    requestBamlCLIVersion()
  }, 30000)

  // TODO: Reactivate linter.
  // runDiagnostics();
}

export function deactivate(): void {
  console.log('BAML extension deactivating')
  diagnosticsCollection.clear()
  diagnosticsCollection.dispose()
  StatusBarPanel.instance.dispose()
  statusBarItem.dispose()
  for (const plugin of plugins) {
    if (plugin.deactivate) {
      void plugin.deactivate()
    }
  }
  server?.close()
}
class DiagnosticCodeActionProvider implements vscode.CodeActionProvider {
  public provideCodeActions(
    document: vscode.TextDocument,
    range: vscode.Range,
    context: vscode.CodeActionContext,
    token: vscode.CancellationToken,
  ): vscode.ProviderResult<vscode.CodeAction[]> {
    const codeActions: vscode.CodeAction[] = []

    for (const diagnostic of context.diagnostics) {
      if (diagnostic.code?.toString().startsWith('[linter]')) {
        const fixString = diagnostic.code.toString().replace('[linter]', '')
        const fixAction = new vscode.CodeAction(`Apply fix: ${fixString}`, vscode.CodeActionKind.QuickFix)
        fixAction.edit = new vscode.WorkspaceEdit()
        fixAction.diagnostics = [diagnostic]
        fixAction.isPreferred = true

        const edit = new vscode.TextEdit(diagnostic.range, fixString)
        fixAction.edit.set(document.uri, [edit])

        codeActions.push(fixAction)
      }
    }
    return codeActions
  }
}
