'use client'

import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from '@/components/ui/resizable'
import { useKeybindingOverrides } from '@/hooks/command-s'
import type { BAMLProject } from '@/lib/exampleProjects'
import { CustomErrorBoundary } from '@baml/playground-common'
import { useAtom, useAtomValue, useSetAtom } from 'jotai'
import { Suspense, useEffect, useRef, useState } from 'react'
import { isMobile } from 'react-device-detect'
import { Editable } from '../../_components/EditableText'
import { JotaiProvider } from '@baml/playground-common'
import {
  activeFileNameAtom,
  currentEditorFilesAtom,
  exploreProjectsOpenAtom,
  unsavedChangesAtom,
} from '../_atoms/atoms'

import FileViewer from './Tree/FileViewer'
import { ScrollArea } from '@/components/ui/scroll-area'
import { filesAtom } from '@/shared/baml-project-panel/atoms'
import { runtimeStateAtom, selectedFunctionAtom } from '@/shared/baml-project-panel/playground-panel/atoms'
import { useFeedbackWidget } from '@baml/playground-common/lib/feedback_widget'
import { TopNavbar } from './TopNavbar'
import Image from 'next/image'
import dynamic from 'next/dynamic'
const CodeMirrorViewer = dynamic(() => import('@baml/playground-common').then((mod) => mod.CodeMirrorViewer), {
  ssr: false,
})
const PromptPreview = dynamic(() => import('@baml/playground-common').then((mod) => mod.PromptPreview), {
  ssr: false,
})
const EventListener = dynamic(() => import('@baml/playground-common').then((mod) => mod.EventListener), {
  ssr: false,
})

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
      setFiles(
        project.files.reduce((acc, f) => {
          acc[f.path] = f.content
          return acc
        }, {} as Record<string, string>),
      )
    }
  }, [project.id])
  const [projectName, setProjectName] = useState(project.name)
  const projectNameInputRef = useRef(null)
  const [description, setDescription] = useState(project.description)
  const descriptionInputRef = useRef(null)

  return (
    // firefox wont apply the background color for some reason so we forcefully set it.
    <div className="flex relative flex-row w-full h-full main-panel overflow-x-clip overflow-y-clip">
      <CustomErrorBoundary message="Error loading project">
        <EventListener>
          {/* noop for now -- we dont need to nest all the other components in the EventListener since we use jotaiprovider store and we dont want to rerender needlessly */}
          <div></div>
        </EventListener>
        {isMobile && (
          <div className="absolute bottom-0 left-0 right-0 font-semibold  border-t-[1px] w-full h-[100px] z-50 text-center p-8">
            Visit PromptFiddle on Desktop to get the best experience
          </div>
        )}
        <ResizablePanelGroup className="w-full h-full overflow-clip" direction="horizontal">
          {!isMobile && <ProjectSidebar />}

          <ResizableHandle className="" />
          <ResizablePanel defaultSize={88}>
            <div className="flex-col w-full h-full font-sans">
              <TopNavbar
                project={project}
                projectName={projectName}
                setProjectName={setProjectName}
                projectNameInputRef={projectNameInputRef}
                unsavedChanges={unsavedChanges}
              />
              <div
                style={{
                  // the size of the topnavbar
                  height: 'calc(100% - 55px)',
                }}
                className="flex flex-row h-full overflow-clip"
              >
                <ResizablePanelGroup className="min-h-[200px] w-full rounded-lg overflow-clip" direction="horizontal">
                  <ResizablePanel defaultSize={50}>
                    <div className="flex flex-col py-1 pl-2 w-full text-xs whitespace-nowrap border-none items-left h-fit">
                      <Editable
                        text={description}
                        placeholder="Write a task name"
                        type="input"
                        childRef={descriptionInputRef}
                        className="px-2 py-2 w-full text-sm font-normal text-left border-none text-foreground"
                      >
                        <textarea
                          className="w-[95%] ml-2 px-2 text-sm border-none"
                          ref={descriptionInputRef}
                          name="task"
                          placeholder="Write a description"
                          value={description}
                          onChange={(e) => setDescription(e.target.value)}
                        />
                      </Editable>
                    </div>
                    <div className="flex pl-1 w-full h-full tour-editor dark:bg-muted/70">
                      <ScrollArea className="w-full h-full">
                        {activeFileName && (
                          <CodeMirrorViewer
                            lang="baml"
                            fileContent={{
                              code: files[activeFileName],
                              language: 'baml',
                              id: activeFileName,
                            }}
                            shouldScrollDown={false}
                            onContentChange={(v) => {
                              const newFiles: Record<string, string> = {}
                              Object.entries(files).map(([key, value]) => {
                                const newVal = key === activeFileName ? v : value
                                newFiles[key] = newVal
                              })
                              setFiles(newFiles)
                            }}
                          />
                        )}
                      </ScrollArea>
                    </div>
                  </ResizablePanel>
                  <ResizableHandle className="" />
                  {!isMobile && (
                    <ResizablePanel defaultSize={50} className="tour-playground">
                      <div className="flex flex-row h-full">
                        <PlaygroundView />
                      </div>
                    </ResizablePanel>
                  )}
                </ResizablePanelGroup>
              </div>
            </div>
          </ResizablePanel>
        </ResizablePanelGroup>
        <FunctionSelectorProvider />
      </CustomErrorBoundary>
    </div>
  )
}

export const FunctionSelectorProvider = () => {
  const activeFileName = useAtomValue(activeFileNameAtom)

  const { functions } = useAtomValue(runtimeStateAtom)
  const editorFiles = useAtomValue(currentEditorFilesAtom)
  const stringifiedEditorFilePaths = JSON.stringify(editorFiles.map((f) => f.path))
  const setSelectedFunction = useSetAtom(selectedFunctionAtom)

  useEffect(() => {
    const func = functions.find((f) => f.span.file_path === activeFileName)
    if (func) {
      setSelectedFunction(func.name)
    }
  }, [stringifiedEditorFilePaths, activeFileName, functions])
  return null
}

export const ProjectSidebar = () => {
  return (
    <ResizablePanel defaultSize={16} className=" h-full dark:bg-[#020309] bg-muted">
      <div className="flex flex-row justify-center items-center pt-4 w-full">
        <a href={'/'} className="flex flex-row items-center text-lg font-semibold text-center w-fit text-foreground">
          <Image src="/baml-lamb-white.png" alt="Prompt Fiddle" width={40} height={40} />
          Prompt Fiddle
        </a>
      </div>

      <ResizablePanelGroup className="pb-4 h-full" direction="vertical">
        <ResizablePanel defaultSize={100} className="h-full">
          <div className="px-2 pt-4 w-full text-xs font-normal text-center uppercase text-muted-foreground">
            project files
          </div>
          <div className="flex flex-col pb-8 w-full h-full tour-file-view">
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
      <JotaiProvider>
        <ProjectViewImpl project={project} />
      </JotaiProvider>
    </>
  )
}

const PlaygroundView = () => {
  return (
    <>
      <CustomErrorBoundary message="Error loading playground">
        <Suspense fallback={<div>Loading...</div>}>
          <div className="flex flex-col w-full h-full">
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
