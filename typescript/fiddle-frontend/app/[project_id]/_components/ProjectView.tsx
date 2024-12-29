'use client'

import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from '@/components/ui/resizable'
import { useKeybindingOverrides } from '@/hooks/command-s'
import type { BAMLProject } from '@/lib/exampleProjects'
import { CodeMirrorViewer, CustomErrorBoundary, PromptPreview } from '@baml/playground-common'
// import { updateFileAtom } from '@baml/playground-common/baml_wasm_web/EventListener'
import { useAtom, useAtomValue, useSetAtom } from 'jotai'
import { useHydrateAtoms } from 'jotai/utils'
import { AlertTriangleIcon, Compass, File, GitForkIcon, LinkIcon } from 'lucide-react'
import Image from 'next/image'
import Link from 'next/link'
import { usePathname } from 'next/navigation'
import posthog from 'posthog-js'
import { Suspense, useEffect, useRef, useState } from 'react'
import { isMobile } from 'react-device-detect'
import { toast } from 'sonner'
import { Editable } from '../../_components/EditableText'
import { type EditorFile, createUrl } from '../../actions'
import {
  activeFileNameAtom,
  currentEditorFilesAtom,
  exploreProjectsOpenAtom,
  unsavedChangesAtom,
} from '../_atoms/atoms'

import { EventListener } from '@baml/playground-common/baml_wasm_web/EventListener'
import { GithubStars } from './GithubStars'

// import SettingsDialog, { ShowSettingsButton } from '@baml/playground-common/shared/SettingsDialog'

import FileViewer from './Tree/FileViewer'
// import { AppStateProvider } from '@baml/playground-common/shared/AppStateContext' // Import the AppStateProvider
// import { ViewSelector } from '@baml/playground-common/shared/Selectors'
import { useFeedbackWidget } from '@baml/playground-common/lib/feedback_widget'
import { filesAtom } from '@/shared/baml-project-panel/atoms'
import { TopNavbar } from './TopNavbar'
import { ScrollArea } from '@/components/ui/scroll-area'
import { runtimeStateAtom, selectedFunctionAtom } from '@/shared/baml-project-panel/playground-panel/atoms'

const ProjectViewImpl = ({ project }: { project: BAMLProject }) => {
  useFeedbackWidget()
  useKeybindingOverrides()
  // Tried to use url pathnames for this but nextjs hijacks the pathname state (even the window.location) so we have to manually track unsaved changes in the app.
  const [files, setFiles] = useAtom(filesAtom)
  const [unsavedChanges, setUnsavedChanges] = useAtom(unsavedChangesAtom)
  const activeFileName = useAtomValue(activeFileNameAtom)

  useEffect(() => {
    if (project) {
      console.log('Updating files due: project', project.id)
      setUnsavedChanges(false)
      console.log('project.files', project.files)
      setFiles(
        project.files.reduce(
          (acc, f) => {
            acc[f.path] = f.content
            return acc
          },
          {} as Record<string, string>,
        ),
      )
    }
  }, [project.id])
  const [projectName, setProjectName] = useState(project.name)
  const projectNameInputRef = useRef(null)
  const [description, setDescription] = useState(project.description)
  const descriptionInputRef = useRef(null)
  const setOpenExplorePanel = useSetAtom(exploreProjectsOpenAtom)
  const setSelectedFunction = useSetAtom(selectedFunctionAtom)
  const { functions } = useAtomValue(runtimeStateAtom)
  const editorFiles = useAtomValue(currentEditorFilesAtom)
  const stringifiedEditorFilePaths = JSON.stringify(editorFiles.map((f) => f.path))

  useEffect(() => {
    const func = functions.find((f) => f.span.file_path === activeFileName)
    if (func) {
      setSelectedFunction(func.name)
    }
  }, [stringifiedEditorFilePaths, activeFileName, functions])

  useEffect(() => {
    console.log('activeFileName', activeFileName)
    if (activeFileName) {
    }
  }, [activeFileName])

  return (
    // firefox wont apply the background color for some reason so we forcefully set it.
    <div className='flex relative flex-row w-full h-full bg-gray-800 main-panel overflow-x-clip overflow-y-clip'>
      <EventListener>
        {isMobile && (
          <div className='absolute bottom-0 left-0 right-0 text-zinc-900 font-semibold bg-zinc-400 border-t-zinc-600 border-t-[1px] w-full h-[100px] z-50 text-center p-8'>
            Visit PromptFiddle on Desktop to get the best experience
          </div>
        )}
        <ResizablePanelGroup className='w-full h-full overflow-clip' direction='horizontal'>
          {!isMobile && <ProjectSidebar />}

          <ResizableHandle className='bg-vscode-contrastActiveBorder border-vscode-contrastActiveBorder' />
          <ResizablePanel defaultSize={88}>
            <div className='flex-col w-full h-full font-sans bg-background dark:bg-vscode-panel-background'>
              <TopNavbar
                project={project}
                projectName={projectName}
                setProjectName={setProjectName}
                projectNameInputRef={projectNameInputRef}
                unsavedChanges={unsavedChanges}
              />
              <div
                style={{
                  height: 'calc(100% - 40px)',
                }}
                className='flex flex-row h-full overflow-clip'
              >
                <ResizablePanelGroup className='min-h-[200px] w-full rounded-lg overflow-clip' direction='horizontal'>
                  <ResizablePanel defaultSize={50}>
                    <div className='flex flex-col py-1 pl-2 w-full text-xs whitespace-nowrap border-none items-left h-fit'>
                      <Editable
                        text={description}
                        placeholder='Write a task name'
                        type='input'
                        childRef={descriptionInputRef}
                        className='px-2 w-full text-sm font-light text-left border-none text-card-foreground/80'
                      >
                        <textarea
                          className='w-[95%] ml-2 px-2 text-sm border-none text-vscode-descriptionForeground'
                          ref={descriptionInputRef}
                          name='task'
                          placeholder='Write a description'
                          value={description}
                          onChange={(e) => setDescription(e.target.value)}
                        />
                      </Editable>
                    </div>
                    <div className='flex w-full h-full tour-editor'>
                      <ScrollArea className='w-full h-full'>
                        {activeFileName && (
                          <CodeMirrorViewer
                            lang='baml'
                            fileContent={{
                              code: files[activeFileName],
                              language: 'baml',
                              id: activeFileName,
                            }}
                            shouldScrollDown={false}
                            onContentChange={() => {}}
                          />
                        )}
                      </ScrollArea>
                    </div>
                  </ResizablePanel>
                  <ResizableHandle className='bg-vscode-tab-activeBackground' />
                  {!isMobile && (
                    <ResizablePanel defaultSize={50} className='tour-playground'>
                      <div className='flex flex-row h-full bg-vscode-panel-background'>
                        <PlaygroundView />
                      </div>
                    </ResizablePanel>
                  )}
                </ResizablePanelGroup>
              </div>
            </div>
          </ResizablePanel>
        </ResizablePanelGroup>
      </EventListener>
    </div>
  )
}

export const ProjectSidebar = () => {
  return (
    <ResizablePanel defaultSize={12} className='h-full bg-zinc-900'>
      <div className='flex flex-row justify-center items-center pt-2 w-full'>
        <a href={'/'} className='flex text-lg italic font-bold text-center w-fit'>
          Prompt Fiddle
        </a>
      </div>

      <ResizablePanelGroup className='pb-4 h-full' direction='vertical'>
        <ResizablePanel defaultSize={100} className='h-full'>
          <div className='px-2 pt-4 w-full text-sm font-semibold text-center uppercase text-white/90'>
            project files
          </div>
          <div className='flex flex-col pb-8 w-full h-full tour-file-view'>
            <FileViewer />
          </div>
        </ResizablePanel>
      </ResizablePanelGroup>
    </ResizablePanel>
  )
}

export const ProjectView = ({ project }: { project: BAMLProject }) => {
  return (
    <>
      <ProjectViewImpl project={project} />
    </>
  )
}

const PlaygroundView = () => {
  return (
    <>
      <CustomErrorBoundary message='Error loading playground'>
        <Suspense fallback={<div>Loading...</div>}>
          <div className='flex flex-col w-full h-full'>
            <PromptPreview />
          </div>

          {/* <InitialTour /> */}
          {/* <PostTestRunTour /> */}
        </Suspense>
      </CustomErrorBoundary>
    </>
  )
}

export default ProjectView
