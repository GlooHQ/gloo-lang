import React from 'react'
import { Button } from '@/components/ui/button'
import { RefreshCcw } from 'lucide-react'
import { ErrorBoundary, type FallbackProps } from 'react-error-boundary'

const ErrorFallback: (message?: string) => React.FC<FallbackProps> = (message) => {
  const FB = ({ error, resetErrorBoundary }: FallbackProps) => {
    return (
      <div
        role='alert'
        className='p-4 bg-vscode-notifications-background border border-vscode-notifications-border rounded'
      >
        <div className='flex items-center justify-between mb-4'>
          <p className='text-vscode-foreground font-medium'>{message ?? 'Something went wrong'}</p>
          <Button onClick={resetErrorBoundary} variant='outline' className='hover:bg-vscode-button-hoverBackground'>
            <RefreshCcw className='w-4 h-4' />
            Reload
          </Button>
        </div>

        {error instanceof Error && (
          <div className='space-y-2'>
            {error.message && (
              <pre className='p-3 bg-vscode-editor-background border border-vscode-panel-border rounded text-sm whitespace-pre-wrap'>
                {error.message}
              </pre>
            )}
            {error.stack && (
              <pre className='p-3 bg-vscode-editor-background border border-vscode-panel-border rounded text-sm whitespace-pre-wrap'>
                {error.stack}
              </pre>
            )}
            {error && Object.keys(error).length > 0 && (
              <pre className='p-3 bg-vscode-editor-background border border-vscode-panel-border rounded text-sm whitespace-pre-wrap'>
                {JSON.stringify(error, null, 2)}
              </pre>
            )}
          </div>
        )}
        {error && typeof error === 'string' && (
          <pre className='p-3 bg-vscode-editor-background border border-vscode-panel-border rounded text-sm whitespace-pre-wrap'>
            {error}
          </pre>
        )}
      </div>
    )
  }
  return FB
}

interface MyErrorBoundaryProps {
  children: React.ReactNode
  message?: string
}

const CustomErrorBoundary: React.FC<MyErrorBoundaryProps> = ({ children, message }) => {
  return (
    <ErrorBoundary
      FallbackComponent={ErrorFallback(message)}
      onReset={() => {
        // Reset the state of your app so the error doesn't happen again
      }}
    >
      {children}
    </ErrorBoundary>
  )
}

export default CustomErrorBoundary
