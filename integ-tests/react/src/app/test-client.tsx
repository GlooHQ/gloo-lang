'use client'

import * as React from 'react'
import { useFnClassOptionalOutput, useLLM, useTestAws } from '../../baml_client/react/client'
import { Loader2 } from "lucide-react"
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Alert, AlertDescription } from "@/components/ui/alert"
import { TestAwsAction } from '../../baml_client/react/server'

export default function TestClient() {
  const {
    data: finalResponse,
    partialData: streamingResponse,
    isLoading,
    isError,
    // isSuccess,
    status,
    error,
    mutate
  } = useTestAws({
    stream: true,
    onPartial: (partial) => {
      console.log('Got partial response', partial)
    },
    onFinal: (final) => {
      console.log('Got final response', final)
    },
    onError: (error) => {
      console.error('Got error', error)
    },
  })

  const {
    data: data2,
    partialData: partialData2,
  } = useLLM(TestAwsAction,{
    stream: true,
    onPartial: (partial) => {
      console.log('Got partial response', partial)
    },
    onFinal: (final) => {
      console.log('Got final response', final)
    },
    onError: (error) => {
      console.error('Got error', error)
    },
  })

  console.log('data2', data2)
  console.log('partialData2', partialData2)
  const response = isLoading ? streamingResponse : finalResponse
  const [prompt, setPrompt] = React.useState('')
  console.log('response', response)

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    if (!prompt.trim()) return

    await mutate(prompt)
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
                  {response}
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
