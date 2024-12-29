/* eslint-disable @typescript-eslint/require-await */
import { useAtomValue } from 'jotai'
import { wasmAtom } from '../../atoms'
import { isDebugModeAtom } from './render-text'
import { useState } from 'react'
import useSWR from 'swr'
import { ExternalLinkIcon, ImageIcon, Music } from 'lucide-react'
// import Link from "next/link";
import type { WasmChatMessagePartMedia } from '@gloo-ai/baml-schema-wasm-web'

interface WebviewMediaProps {
  bamlMediaType: 'image' | 'audio'
  media: WasmChatMessagePartMedia
}

export const WebviewMedia: React.FC<WebviewMediaProps> = ({ bamlMediaType, media }) => {
  const wasm = useAtomValue(wasmAtom)
  const isDebugMode = useAtomValue(isDebugModeAtom)
  const [imageStats, setImageStats] = useState<{
    width: number
    height: number
    size: string
  }>()

  const {
    data: mediaUrl,
    error,
    isLoading,
  } = useSWR({ swr: 'WebviewMedia', type: media.type, content: media.content }, async () => {
    if (!wasm) {
      throw new Error('wasm not loaded')
    }

    switch (media.type) {
      case wasm.WasmChatMessagePartMediaType.File:
        return `${media.content}`
      case wasm.WasmChatMessagePartMediaType.Url:
        return media.content
      case wasm.WasmChatMessagePartMediaType.Error:
        throw new Error(media.content)
      default:
        throw new Error('unknown media type')
    }
  })

  if (error) {
    return (
      <div className='px-4 py-3 rounded-lg bg-destructive/15 text-destructive'>
        <p className='text-sm font-medium'>Error loading {bamlMediaType}</p>
        <p className='mt-1 text-xs'>{error.message}</p>
      </div>
    )
  }

  if (isLoading) {
    return (
      <div className='flex h-[200px] items-center justify-center rounded-lg bg-muted'>
        <p className='text-sm text-muted-foreground'>Loading {bamlMediaType}...</p>
      </div>
    )
  }

  const onImageLoad = (e: React.SyntheticEvent<HTMLImageElement>) => {
    const img = e.currentTarget
    const { naturalWidth, naturalHeight } = img
    let size = 'Unknown'
    if (mediaUrl?.startsWith('data:')) {
      const base64Length = mediaUrl.split(',')[1]?.length
      const sizeInBytes = base64Length ? base64Length * 0.75 : 0
      size =
        sizeInBytes > 1048576 ? `${(sizeInBytes / 1048576).toFixed(2)} MB` : `${(sizeInBytes / 1024).toFixed(2)} KB`
    } else {
      const sizeInBytes = naturalWidth * naturalHeight * 4
      size =
        sizeInBytes > 1048576 ? `${(sizeInBytes / 1048576).toFixed(2)} MB` : `${(sizeInBytes / 1024).toFixed(2)} KB`
    }
    setImageStats({ width: naturalWidth, height: naturalHeight, size })
  }

  return (
    <div className='w-full'>
      {isDebugMode && bamlMediaType === 'image' && (
        <div className='flex flex-row gap-4 justify-start items-center px-3 py-2 text-xs border-b border-border bg-muted text-muted-foreground'>
          <div className='flex items-center gap-1.5'>
            <span className='text-muted-foreground/60'>Dimensions:</span>
            <span className='font-medium'>
              {imageStats?.width ?? '?'}×{imageStats?.height ?? '?'}
            </span>
          </div>
          <div className='flex items-center gap-1.5'>
            <span className='text-muted-foreground/60'>Size:</span>
            <span className='font-medium'>{imageStats?.size ?? '?'}</span>
          </div>
          <div className='flex items-center gap-1.5'>
            <span className='text-muted-foreground/60'>Tokens (est.):</span>
            <span className='font-medium'>
              {Math.ceil(((imageStats?.width ?? 0) * (imageStats?.height ?? 0)) / 750)}
            </span>
          </div>
        </div>
      )}
      <div className='relative w-fit'>
        <div className='flex gap-2 items-center px-2 py-1 w-full text-xs text-white rounded-t-lg bg-black/50'>
          {bamlMediaType === 'image' ? <ImageIcon className='w-3 h-3' /> : <Music className='w-3 h-3' />}
          {mediaUrl && (
            <a
              href={mediaUrl}
              target='_blank'
              rel='noopener noreferrer'
              className='flex gap-1 items-center transition-colors hover:text-blue-300'
            >
              <ExternalLinkIcon className='w-3 h-3' />
              <span className='max-w-[150px] truncate'>{mediaUrl}</span>
            </a>
          )}
        </div>
        {bamlMediaType === 'image' ? (
          <img
            src={mediaUrl}
            alt={`Image Not Found`}
            className='mx-auto max-h-[400px] max-w-[400px] rounded-b-lg object-contain'
            onLoad={onImageLoad}
          />
        ) : (
          <audio controls className='p-2 w-full'>
            <source src={mediaUrl} />
            Your browser does not support the audio element.
          </audio>
        )}
      </div>
    </div>
  )
}
