'use client'

import * as React from 'react'
import { useTestAws } from '../../baml_client/react/client'
import { Loader2 } from "lucide-react"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Alert, AlertDescription } from "@/components/ui/alert"

export default function TestClient() {
  const {
    data: finalResponse,
    partialData: streamingResponse,
    isLoading,
    isError,
    error,
    mutate
  } = useTestAws()

  const response = isLoading ? streamingResponse : finalResponse
  const [prompt, setPrompt] = React.useState('')

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault()
    if (!prompt.trim()) return

    await mutate({ input: prompt })
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

            <p className="text-sm text-muted-foreground text-center">
              {isLoading ? 'Streaming response...' : 'Final response'}
            </p>
          </div>
        )}
      </CardContent>
    </Card>
  )
}
