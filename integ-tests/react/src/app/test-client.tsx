'use client'

import * as React from 'react'
import { useLLM, useTestAws, useTestUniverseQuestion} from '../../baml_client/react/client'
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { TestAwsAction}  from '../../baml_client/react/server'
import { NonStreamingReturnType } from '../../baml_client/react/types'
import { StreamingReturnType } from '../../baml_client/react/types'
import { Loader2 } from 'lucide-react'
import { Alert, AlertDescription } from '@/components/ui/alert'

export default function TestClient() {

  const streamingDirectAction = useTestAws({
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

  // Streaming should not have errors
  streamingDirectAction satisfies StreamingReturnType<string, [string]>;
  streamingDirectAction.data satisfies string | undefined;
  streamingDirectAction.partialData satisfies string | null | undefined;
  streamingDirectAction.mutate satisfies (input: string) => Promise<ReadableStream<Uint8Array>>;

  // Non-Streaming should have errors
  streamingDirectAction satisfies NonStreamingReturnType<string, [string]>;
  streamingDirectAction.data satisfies never;
  streamingDirectAction.partialData satisfies never;
  streamingDirectAction.mutate satisfies (input: string) => Promise<string>;

  const nonStreamingDirectAction = useTestAws({
    onFinal: (response) => {
      console.log('Got final response', response)
    },
    onError: (error) => {
      console.error('Got error', error)
    },
  })

  // Streaming should have errors
  nonStreamingDirectAction satisfies StreamingReturnType<string, [string]>;
  nonStreamingDirectAction.data satisfies never;
  nonStreamingDirectAction.partialData satisfies string | null;
  nonStreamingDirectAction.mutate satisfies (input: string) => Promise<ReadableStream<Uint8Array>>;

  // Non-Streaming should not have errors
  nonStreamingDirectAction satisfies NonStreamingReturnType<string, [string]>;
  nonStreamingDirectAction.data satisfies string | undefined;
  nonStreamingDirectAction.partialData satisfies never | undefined;
  nonStreamingDirectAction.mutate satisfies (input: string) => Promise<string>;


  const streamingIndirectAction = useLLM(TestAwsAction, {
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

  // Streaming should not have errors
  streamingIndirectAction satisfies StreamingReturnType<string, [string]>;
  streamingIndirectAction.data satisfies string | undefined;
  streamingIndirectAction.partialData satisfies string | null | undefined;
  streamingIndirectAction.mutate satisfies (input: string) => Promise<ReadableStream<Uint8Array>>;

  // Non-Streaming should have errors
  streamingIndirectAction satisfies NonStreamingReturnType<string, [string]>;
  streamingIndirectAction.data satisfies never;
  streamingIndirectAction.partialData satisfies never | undefined;
  streamingIndirectAction.mutate satisfies (input: string) => Promise<string>;

  const nonStreamingIndirectAction = useLLM(TestAwsAction, {
    onFinal: (response) => {
      console.log('Got final response', response)
    },
    onError: (error) => {
      console.error('Got error', error)
    },
  })

  // Streaming should have errors
  nonStreamingIndirectAction satisfies StreamingReturnType<string, [string]>;
  nonStreamingIndirectAction.data satisfies never
  nonStreamingIndirectAction.partialData satisfies never;
  nonStreamingIndirectAction.mutate satisfies (input: string) => Promise<ReadableStream<Uint8Array>>;

  // Non-Streaming should not have errors
  nonStreamingIndirectAction satisfies NonStreamingReturnType<string, [string]>;
  nonStreamingIndirectAction.data satisfies string | undefined;
  nonStreamingIndirectAction.partialData satisfies never | undefined;
  nonStreamingIndirectAction.mutate satisfies (input: string) => Promise<string>;

  const universeAction  = useTestUniverseQuestion({
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

  const {isPending, error, isError, isSuccess, mutate, status, data, partialData} = universeAction;

  const response = isPending? partialData: data
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
              disabled={isPending}
            />
          </div>

          <Button
            type="submit"
            className="w-full"
            disabled={isPending || !prompt.trim()}
          >
            {isPending && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            {isPending ? 'Processing...' : 'Submit'}
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
