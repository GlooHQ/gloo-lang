/// Content once a function has been selected.

import { ParserDatabase, TestResult, TestStatus } from '@baml/common'
import { useImplCtx, useSelections } from './hooks'
import {
  VSCodeBadge,
  VSCodeCheckbox,
  VSCodePanelTab,
  VSCodePanelView,
  VSCodePanels,
  VSCodeProgressRing,
} from '@vscode/webview-ui-toolkit/react'
import { useMemo, useState } from 'react'
import Link from './Link'
import TypeComponent from './TypeComponent'
import { Impl } from '@baml/common/src/parser_db'
import clsx from 'clsx'
import { Tiktoken, encodingForModel } from "js-tiktoken";


const Whitespace: React.FC<{ char: 'space' | 'tab' }> = ({ char }) => (
  <span className="opacity-50 text-vscode-descriptionForeground">{char === 'space' ? <>&middot;</> : <>&rarr;</>}</span>
)

const InvisibleUtf: React.FC<{ text: string }> = ({ text }) => (
  <span className="text-xs text-red-500 opacity-75">
    {text
      .split('')
      .map((c) => `U+${c.charCodeAt(0).toString(16).padStart(4, '0')}`)
      .join('')}
  </span>
)

// Excludes 0x20 (space) and 0x09 (tab)
const VISIBLE_WHITESPACE = /\u0020\u0009/
const INVISIBLE_CODES =
  /\u00a0\u00ad\u034f\u061c\u070f\u115f\u1160\u1680\u17b4\u17b5\u180e\u2000\u2001\u2002\u2003\u2004\u2005\u2006\u2007\u2008\u2009\u200a\u200b\u200c\u200d\u200e\u200f\u202f\u205f\u2060\u2061\u2062\u2063\u2064\u206a\u206b\u206c\u206d\u206e\u206f\u3000\u2800\u3164\ufeff\uffa0/
const whitespaceRegexp = new RegExp(`([${VISIBLE_WHITESPACE}]+|[${INVISIBLE_CODES}]+)`, 'g')

const TOKEN_BG_STYLES = [
  "bg-fuchsia-800",
  "bg-emerald-700",
  "bg-yellow-600",
  "bg-red-700",
  "bg-cyan-700",
]

// Function to replace whitespace characters with visible characters
const replaceWhitespace = (char: string, key: string) => {
  if (char === ' ') return <Whitespace key={key} char="space" />
  if (char === '\t') return <Whitespace key={key} char="tab" />
  return char
}

const renderLine = ({text, showWhitespace, wrapText}: {text: string, showWhitespace: boolean, wrapText: boolean}) => {
  // Split the text into segments
  const segments = text.split(whitespaceRegexp)

  // Map segments to appropriate components or strings
  const formattedText = segments.map((segment, index) => {
    if (showWhitespace && new RegExp(`^[${VISIBLE_WHITESPACE}]+$`).test(segment)) {
      return segment.split('').map((char, charIndex) => replaceWhitespace(char, index.toString() + charIndex))
    } else if (new RegExp(`^[${INVISIBLE_CODES}]+$`).test(segment)) {
      return <InvisibleUtf key={index} text={segment} />
    } else {
      return segment
    }
  })
  return showWhitespace ? (
    <div className={clsx('flex text-xs inline-block', { 'flex-wrap': wrapText })}>
      {formattedText}
    </div>
  ) : (
    <>{formattedText}</>
  )
}

const CodeLine: React.FC<{ 
  // line is either the entire line, or when tokenization is on, an array of [token, tokenIndex]
  line: string | [string, number][];
  lineNumber: number;
  showWhitespace: boolean;
  wrapText: boolean;
  maxLineNumber: number;
}> = ({
  line,
  lineNumber: lineNumber,
  showWhitespace,
  wrapText,
  maxLineNumber,
}) => {
  // Function to render whitespace characters and invisible UTF characters with special styling
  const lineNumberSpan = (
    <span className="pr-2 font-mono text-xs text-right text-gray-500 select-none">
      {lineNumber.toString().padStart(maxLineNumber.toString().length, ' ')}
    </span>
  );

  if (Array.isArray(line)) {
    return (
      <div>
        {lineNumberSpan}
        {line.map(([token, tokenIndex], index) => (
          <span
            className={clsx('font-mono text-xs inline-block', TOKEN_BG_STYLES[tokenIndex % TOKEN_BG_STYLES.length],
            {
              'whitespace-pre-wrap': wrapText,
              "after:content-['↵']": index === line.length - 1,
              "after:opacity-50": index === line.length - 1,
            },)}
          >
            {renderLine({text: token, showWhitespace, wrapText})}
          </span>
        ))}
      </div>
    )
  }

  return (
    <div>
      {lineNumberSpan}
      <span
        className={clsx('font-mono text-xs inline-block', { 'whitespace-pre-wrap': wrapText })}
      >
        {renderLine({text: line, showWhitespace, wrapText})}
      </span>
    </div>
  )
}

const gpt4Enc = encodingForModel("gpt-4")

const Snippet: React.FC<{ text: string, type?: "preview" | "error" }> = ({ text, type = "preview" } ) => {
  const [showTokens, setShowTokens] = useState(false)
  const [showWhitespace, setShowWhitespace] = useState(false)
  const [wrapText, setWrapText] = useState(true)

  const [enc, tokens] = useMemo(() => {
    if (!showTokens) return [undefined, undefined]

    return [gpt4Enc, gpt4Enc.encode(text)]
  }, [text, showTokens]);

  const divStyle = clsx("r-full", "p-1", "overflow-hidden", "rounded-lg", {
    "bg-vscode-input-background": type === "preview",
    "bg-vscode-inputValidation-errorBackground" : type === "error",
  });

  const header = (
    <div className="flex flex-row justify-end gap-2 text-xs">
      {showTokens && (<div className="flex-grow r-full ps-2 pt-1.5">Tokens: {(tokens as []).length}</div>)}
      <VSCodeCheckbox
        checked={showTokens}
        onChange={(e) => setShowTokens((e as React.FormEvent<HTMLInputElement>).currentTarget.checked)}
      >
        Show Tokens
      </VSCodeCheckbox>
      <VSCodeCheckbox
        checked={wrapText}
        onChange={(e) => setWrapText((e as React.FormEvent<HTMLInputElement>).currentTarget.checked)}
      >
        Wrap Text
      </VSCodeCheckbox>
      <VSCodeCheckbox
        checked={showWhitespace}
        onChange={(e) => setShowWhitespace((e as React.FormEvent<HTMLInputElement>).currentTarget.checked)}
      >
        Whitespace
      </VSCodeCheckbox>
    </div>
  );

  if (showTokens) {
    const tokenized = Array.from(tokens as number[]).map((token) => (enc as Tiktoken).decode([token]))
    const tokenizedLines: [string, number][][] = [[]]
    tokenized.forEach((token, tokenIndex) => {
      const noNewlines = token.split('\n');
      (tokenizedLines.at(-1) as [string, number][]).push([noNewlines.at(0) as string, tokenIndex])
      for (let i = 1; i < noNewlines.length; i++) {
        tokenizedLines.push([["", tokenIndex]])
      }
    });
    const tokenizedContent = tokenizedLines.map((line, lineIndex) => (
      <CodeLine key={lineIndex} line={line} lineNumber={lineIndex+1} maxLineNumber={tokenizedLines.length} showWhitespace={false} wrapText={false}/>
    ))
    return (
      <div className={divStyle}>
        {header}
        <pre className="w-full p-1 text-xs">
          {tokenizedContent}
        </pre>
      </div>
    );
  } else {
    const lines = text.split('\n')
    return (
      <div className={divStyle}>
        {header}
        <pre className="w-full p-1 text-xs">
          {lines.map((line, index) => (
            <CodeLine
              key={index}
              maxLineNumber={lines.length}
              line={line}
              lineNumber={index + 1}
              showWhitespace={showWhitespace}
              wrapText={wrapText} />
          ))}
        </pre>
      </div>
    );
  }
}

const PromptPreview: React.FC<{ prompt: Impl['prompt'] }> = ({prompt}) => {
  switch (prompt.type) {
    case "Completion":
      return <Snippet text={prompt.completion} />
    case "Chat":
      return (<div className='flex flex-col gap-2'>
              {prompt.chat.map(({ role, message }, index: number) => (
                <div className='flex flex-col'>
                  <div className='text-xs'><span className='text-muted-foreground'>Role:</span> <span className='font-bold'>{role}</span></div>
                  <Snippet key={index} text={message} />
                </div>
              ))}
            </div>);
    case "Error":
      return <Snippet type="error" text={prompt.error} />
  }
}

const ImplPanel: React.FC<{ impl: Impl, showTab: boolean }> = ({ impl, showTab }) => {
  const { func } = useImplCtx(impl.name.value)

  if (!func) return null

  return (
    <>
      {
        showTab && (
          <VSCodePanelTab key={`tab-${impl.name.value}`} id={`tab-${func.name.value}-${impl.name.value}`}>
            <div className="flex flex-row gap-1">
              <span>{impl.name.value}</span>
            </div>
          </VSCodePanelTab>
        )
      }
      <VSCodePanelView key={`view-${impl.name.value}`} id={`view-${func.name.value}-${impl.name.value}`}>
        <div className="flex flex-col w-full gap-2">
          <div className="flex flex-col gap-1">
            <div className="flex flex-row items-center justify-between">
              <span className="flex gap-1">
                <b>Prompt</b>
                <Link item={impl.name} display="Edit" />
              </span>
              <div className="flex flex-row gap-1">
                {/* <span className="font-light">Client</span> */}
                <Link item={impl.client.identifier} />
              </div>
            </div>
            <PromptPreview prompt={impl.prompt}/>
          </div>
        </div>
      </VSCodePanelView>
    </>
  )
}

export default ImplPanel
