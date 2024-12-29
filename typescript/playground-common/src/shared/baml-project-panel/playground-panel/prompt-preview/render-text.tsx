import { atom, useAtomValue } from 'jotai'
import { renderModeAtom } from '../preview-toolbar'
import { useMemo, useState } from 'react'
import { ChevronDown, ChevronUp } from 'lucide-react'
import { ScrollArea } from '@/components/ui/scroll-area'

export const isDebugModeAtom = atom((get) => get(renderModeAtom) === 'tokens')

export const RenderText: React.FC<{
  text: string
  highlightChunks?: string[]
}> = ({ text, highlightChunks = [] }) => {
  const isDebugMode = useAtomValue(isDebugModeAtom)
  const isLongText = useMemo(() => text.split('\n').length > 18, [text])
  const [isFullTextVisible, setIsFullTextVisible] = useState(false)

  const highlightedText = useMemo(() => {
    if (!highlightChunks?.length) return text

    let result = text
    highlightChunks.forEach((chunk) => {
      if (!chunk) return
      const regex = new RegExp(`(${chunk.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'g')
      result = result.replace(
        regex,
        '<mark class="bg-yellow-100/70 text-yellow-900 dark:bg-yellow-800/30 dark:text-yellow-100 rounded-sm px-0.5">$1</mark>',
      )
    })
    return result
  }, [text, highlightChunks])

  return (
    <div className='flex flex-col'>
      {isDebugMode && (
        <div className='flex flex-row gap-4 justify-start items-center px-3 py-2 text-xs border-b border-border bg-muted text-muted-foreground'>
          <div className='flex items-center gap-1.5'>
            <span className='text-muted-foreground/60'>Characters:</span>
            <span className='font-medium'>{text.length}</span>
          </div>
          <div className='flex items-center gap-1.5'>
            <span className='text-muted-foreground/60'>Words:</span>
            <span className='font-medium'>{text.split(/\s+/).filter(Boolean).length}</span>
          </div>
          <div className='flex items-center gap-1.5'>
            <span className='text-muted-foreground/60'>Lines:</span>
            <span className='font-medium'>{text.split('\n').length}</span>
          </div>
          <div className='flex items-center gap-1.5'>
            <span className='text-muted-foreground/60'>Tokens (est.):</span>
            <span className='font-medium'>{Math.ceil(text.length / 4)}</span>
          </div>
        </div>
      )}
      <ScrollArea className='relative flex-1 p-2 pb-6 bg-muted/50 dark:bg-slate-900' type='always'>
        <pre
          className={`whitespace-pre-wrap text-xs  ${isFullTextVisible ? 'max-h-96' : 'max-h-64'}`}
          dangerouslySetInnerHTML={{ __html: highlightedText }}
        />

        {isLongText && (
          <button
            onClick={() => setIsFullTextVisible(!isFullTextVisible)}
            className='flex absolute right-0 bottom-0 gap-1 items-center p-2 text-xs rounded-tr-md rounded-bl-md transition-colors bg-muted/50 text-muted-foreground hover:text-foreground'
          >
            {isFullTextVisible ? (
              <>
                Show less
                <ChevronUp className='w-3 h-3' />
              </>
            ) : (
              <>
                Show more
                <ChevronDown className='w-3 h-3' />
              </>
            )}
          </button>
        )}
      </ScrollArea>
    </div>
  )
}
