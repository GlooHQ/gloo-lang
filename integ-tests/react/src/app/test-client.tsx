'use client'

import * as React from 'react'
import { useAliasWithMultipleAttrs, useBuildLinkedList, useLLM, useTestAws, useTestUniverseQuestion, useUnionTest_Function } from '../../baml_client/react/client'
import { Loader2 } from "lucide-react"
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Alert, AlertDescription } from "@/components/ui/alert"
// import { TestAwsAction } from '../../baml_client/react/server'
// import { NonStreamableServerActionType, NonStreamingReturnType, StreamableServerActionType, StreamingReturnType } from '../../baml_client/react/types'

export default function TestClient() {

  // const streamingDirectAction = useTestAws({
  //   stream: true,
  //   onPartial: (response) => {
  //     console.log('Got partial response', response)
  //   },
  //   onFinal: (response) => {
  //     console.log('Got final response', response)
  //   },
  //   onError: (error) => {
  //     console.error('Got error', error)
  //   },
  // })

  // // Streaming should not have errors
  // streamingDirectAction satisfies StreamingReturnType<string, [string]>;
  // streamingDirectAction.data satisfies string | null;
  // streamingDirectAction.partialData satisfies string | null;
  // streamingDirectAction.mutate satisfies (input: string) => Promise<ReadableStream<Uint8Array>>;

  // // Non-Streaming should have errors
  // streamingDirectAction satisfies NonStreamingReturnType<string, [string]>;
  // streamingDirectAction.data satisfies string | null;
  // streamingDirectAction.partialData satisfies never;
  // streamingDirectAction.mutate satisfies (input: string) => Promise<string>;

  // const nonStreamingDirectAction = useTestAws({
  //   onFinal: (response) => {
  //     console.log('Got final response', response)
  //   },
  //   onError: (error) => {
  //     console.error('Got error', error)
  //   },
  // })

  // // Streaming should have errors
  // nonStreamingDirectAction satisfies StreamingReturnType<string, [string]>;
  // nonStreamingDirectAction.data satisfies string | null;
  // nonStreamingDirectAction.partialData satisfies string | null;
  // nonStreamingDirectAction.mutate satisfies (input: string) => Promise<ReadableStream<Uint8Array>>;

  // // Non-Streaming should not have errors
  // nonStreamingDirectAction satisfies NonStreamingReturnType<string, [string]>;
  // nonStreamingDirectAction.data satisfies string | null;
  // nonStreamingDirectAction.partialData satisfies never;
  // nonStreamingDirectAction.mutate satisfies (input: string) => Promise<string>;

  // const streamingIndirectAction = useLLM(TestAwsAction, {
  //   stream: true,
  //   onPartial: (response) => {
  //     console.log('Got partial response', response)
  //   },
  //   onFinal: (response) => {
  //     console.log('Got final response', response)
  //   },
  //   onError: (error) => {
  //     console.error('Got error', error)
  //   },
  // })

  // // Streaming should not have errors
  // streamingIndirectAction satisfies StreamingReturnType<string, [string]>;
  // streamingIndirectAction.data satisfies string | null;
  // streamingIndirectAction.partialData satisfies string | null;
  // streamingIndirectAction.mutate satisfies (input: string) => Promise<ReadableStream<Uint8Array>>;

  // // Non-Streaming should have errors
  // streamingIndirectAction satisfies NonStreamingReturnType<string, [string]>;
  // streamingIndirectAction.data satisfies string | null;
  // streamingIndirectAction.partialData satisfies never;
  // streamingIndirectAction.mutate satisfies (input: string) => Promise<string>;

  // const nonStreamingIndirectAction = useLLM(TestAwsAction, {
  //   onFinal: (response) => {
  //     console.log('Got final response', response)
  //   },
  //   onError: (error) => {
  //     console.error('Got error', error)
  //   },
  // })

  // // Streaming should have errors
  // nonStreamingIndirectAction satisfies StreamingReturnType<string, [string]>;
  // nonStreamingIndirectAction.data satisfies string | null;
  // nonStreamingIndirectAction.partialData satisfies string | null;
  // nonStreamingIndirectAction.mutate satisfies (input: string) => Promise<ReadableStream<Uint8Array>>;

  // // Non-Streaming should not have errors
  // nonStreamingIndirectAction satisfies NonStreamingReturnType<string, [string]>;
  // nonStreamingIndirectAction.data satisfies string | null;
  // nonStreamingIndirectAction.partialData satisfies never;
  // nonStreamingIndirectAction.mutate satisfies (input: string) => Promise<string>;

  //  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  //  TestAwsAction satisfies StreamableServerActionType<string, [string]>;
  //  TestAwsAction satisfies NonStreamableServerActionType<string, [string]>;

  //  React.useEffect(() => {
  //   const foo3 = TestAwsAction('foo', { })
  //   console.log('foo3', foo3)
  //  }, [])

  const {
    data: finalResponse,
    partialData: streamingResponse,
    isLoading,
    isError,
    status,
    error,
    mutate
  } = useTestUniverseQuestion({
    stream: true,
    onPartial: (response) => {
      console.log('Got partial response', response)
    },
    onFinal: (response) => {
      console.log('Got final response', response)
    },
    onError: (error) => {
      console.error('Got error', error)
    },
  })

  const response = isLoading ? streamingResponse : finalResponse
  const [prompt, setPrompt] = React.useState('')

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    if (!prompt.trim()) return

    await mutate({ question: prompt })
    setPrompt('')
  }

  return (
    <Card className="w-full">
      <CardHeader>
        <CardTitle>BAML AWS Test</CardTitle>
        <CardDescription>
          Test the BAML AWS integration by entering some text below.
        </CardDescription>
      </CardHeader>

      <CardContent>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="prompt">Test Input</Label>
            <Input
              id="prompt"
              type="text"
              value={prompt}
              onChange={(e: React.ChangeEvent<HTMLInputElement>) => setPrompt(e.target.value)}
              placeholder="Type something..."
              disabled={isLoading}
            />
          </div>

          <Button
            type="submit"
            className="w-full"
            disabled={isLoading || !prompt.trim()}
          >
            {isLoading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            {isLoading ? 'Processing...' : 'Submit'}
          </Button>
        </form>

        {isError && (
          <Alert variant="destructive" className="mt-4">
            <AlertDescription>
              Error: {error?.message}
            </AlertDescription>
          </Alert>
        )}

        {response && (
          <div className="mt-6 space-y-2">
            <Card>
              <CardContent className="pt-6">
                <pre className="whitespace-pre-wrap font-mono text-sm bg-muted p-4 rounded-lg">
                  {typeof response === 'string' ? response : JSON.stringify(response, null, 2)}
                </pre>
              </CardContent>
            </Card>
          </div>
        )}
        <CardFooter className="text-sm text-muted-foreground text-center">
          {status}
        </CardFooter>
      </CardContent>
    </Card>
  )
}
