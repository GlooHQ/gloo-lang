import { useAtomValue } from 'jotai'
import { renderModeAtom } from '../preview-toolbar'
import { PromptPreviewCurl } from './prompt-preview-curl'
import { PromptPreviewContent } from './prompt-preview-content'

export const PromptRenderWrapper = () => {
  const renderMode = useAtomValue(renderModeAtom)

  if (renderMode === 'curl') {
    return <PromptPreviewCurl />
  }

  return <PromptPreviewContent />
}
