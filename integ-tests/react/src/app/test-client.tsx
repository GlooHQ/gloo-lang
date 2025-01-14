// NOTE: Uncomment this to verify that the types are working
// @ts-nocheck

'use client'

import { Alert, AlertDescription } from '@/components/ui/alert'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Loader2 } from 'lucide-react'
import * as React from 'react'
import { useBamlAction, useTestAws } from '../../baml_client/react/client'
import { TestAws } from '../../baml_client/react/server_streaming'
import type { HookResult, NonStreamingHookResult, StreamingHookResult } from '../../baml_client/react/types'

type ResponseCardProps = {
  streamingHookResult: StreamingHookResult<'TestAws'>
  nonStreamingHookResult: NonStreamingHookResult<'TestAws'>
  status: HookResult['status']
}

function ResponseCard({ streamingHookResult, nonStreamingHookResult, status }: ResponseCardProps) {
  const { isPending, error, isError, data, partialData } = streamingHookResult
  const response = isPending ? partialData : data

  return (
    <>
      {isError && (
        <Alert variant='destructive' className='mt-4'>
          <AlertDescription>Error: {error?.message}</AlertDescription>
        </Alert>
      )}

      {response && (
        <div className='mt-6 space-y-2'>
          <Card>
            <CardContent className='pt-6'>
              <pre className='whitespace-pre-wrap font-mono text-sm bg-muted p-4 rounded-lg'>
                {typeof response === 'string' ? response : JSON.stringify(response, null, 2)}
              </pre>
            </CardContent>
          </Card>
        </div>
      )}
      <CardFooter className='text-sm text-muted-foreground text-center'>{status}</CardFooter>
    </>
  )
}

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

  // // Streaming should not have errors
  streamingDirectAction satisfies StreamingHookResult<'TestAws'>
  streamingDirectAction.data satisfies string | undefined
  streamingDirectAction.partialData satisfies string | null | undefined
  streamingDirectAction.mutate satisfies (input: string) => Promise<ReadableStream<Uint8Array>>

  // // Non-Streaming should have errors
  streamingDirectAction satisfies NonStreamingHookResult<'TestAws'>
  streamingDirectAction.data satisfies never
  streamingDirectAction.partialData satisfies never
  streamingDirectAction.mutate satisfies (input: string) => Promise<string>

  const nonStreamingDirectAction = useTestAws({
    onFinal: (response) => {
      console.log('Got final response', response)
    },
    onError: (error) => {
      console.error('Got error', error)
    },
  })

  // Streaming should have errors
  nonStreamingDirectAction satisfies NonStreamingHookResult<'TestAws'>
  nonStreamingDirectAction.data satisfies never
  nonStreamingDirectAction.partialData satisfies never
  nonStreamingDirectAction.mutate satisfies (input: string) => Promise<string>

  // Non-Streaming should not have errors
  nonStreamingDirectAction satisfies NonStreamingHookResult<'TestAws'>
  nonStreamingDirectAction.data satisfies string | undefined
  nonStreamingDirectAction.partialData satisfies never | undefined
  nonStreamingDirectAction.mutate satisfies (input: string) => Promise<string>

  const streamingIndirectAction = useBamlAction(TestAws, {
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
  streamingIndirectAction satisfies StreamingHookResult<'TestAws'>
  streamingIndirectAction.data satisfies string | undefined
  streamingIndirectAction.partialData satisfies string | null | undefined
  streamingIndirectAction.mutate satisfies (input: string) => Promise<ReadableStream<Uint8Array>>

  // Non-Streaming should have errors
  streamingIndirectAction satisfies NonStreamingHookResult<'TestAws'>
  streamingIndirectAction.data satisfies never
  streamingIndirectAction.partialData satisfies never | undefined
  streamingIndirectAction.mutate satisfies (input: string) => Promise<string>

  const nonStreamingIndirectAction = useBamlAction(TestAws, {
    onFinal: (response) => {
      console.log('Got final response', response)
    },
    onError: (error) => {
      console.error('Got error', error)
    },
  })

  // Streaming should have errors
  nonStreamingIndirectAction satisfies StreamingHookResult<'TestAws'>
  nonStreamingIndirectAction.data satisfies never
  nonStreamingIndirectAction.partialData satisfies never
  nonStreamingIndirectAction.mutate satisfies (input: string) => Promise<ReadableStream<Uint8Array>>

  // Non-Streaming should not have errors
  nonStreamingIndirectAction satisfies NonStreamingHookResult<'TestAws'>
  nonStreamingIndirectAction.data satisfies string | undefined
  nonStreamingIndirectAction.partialData satisfies never | undefined
  nonStreamingIndirectAction.mutate satisfies (input: string) => Promise<string>

  const { isPending, error, isError, isSuccess, mutate, status, data, partialData } = streamingDirectAction

  const response = isPending ? partialData : data
  const [prompt, setPrompt] = React.useState('')

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    if (!prompt.trim()) return

    await mutate(prompt)
    setPrompt('')
  }

  return (
    <Card className='w-full'>
      <CardHeader>
        <CardTitle>BAML AWS Test</CardTitle>
        <CardDescription>Test the BAML AWS integration by entering some text below.</CardDescription>
      </CardHeader>

      <CardContent>
        <form onSubmit={handleSubmit} className='space-y-4'>
          <div className='space-y-2'>
            <Label htmlFor='prompt'>Test Input</Label>
            <Input
              id='prompt'
              type='text'
              value={prompt}
              onChange={(e: React.ChangeEvent<HTMLInputElement>) => setPrompt(e.target.value)}
              placeholder='Type something...'
              disabled={isPending}
            />
          </div>

          <Button type='submit' className='w-full' disabled={isPending || !prompt.trim()}>
            {isPending && <Loader2 className='mr-2 h-4 w-4 animate-spin' />}
            {isPending ? 'Processing...' : 'Submit'}
          </Button>
        </form>

        <ResponseCard
          streamingHookResult={streamingDirectAction}
          nonStreamingHookResult={nonStreamingDirectAction}
          status={status}
        />
      </CardContent>
    </Card>
  )
}
