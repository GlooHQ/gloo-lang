'use client'

import { ExampleProjectCard } from '@/app/_components/ExampleProjectCard'
import { Badge } from '@/components/ui/badge'
import { Button, buttonVariants } from '@/components/ui/button'
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from '@/components/ui/resizable'
import { ScrollArea } from '@/components/ui/scroll-area'
import { useKeybindingOverrides } from '@/hooks/command-s'
import { BAML_DIR } from '@/lib/constants'
import type { BAMLProject } from '@/lib/exampleProjects'
import {
  CustomErrorBoundary,
  // FunctionPanel,
  //useSelections,
} from '@baml/playground-common'
// import { updateFileAtom } from '@baml/playground-common/baml_wasm_web/EventListener'
import { Separator } from '@baml/playground-common/components/ui/separator'
import clsx from 'clsx'
import { useAtom, useAtomValue, useSetAtom } from 'jotai'
import { useHydrateAtoms } from 'jotai/utils'
import { AlertTriangleIcon, Compass, File, FlaskConical, GitForkIcon, LinkIcon, ShareIcon } from 'lucide-react'
import Image from 'next/image'
import Link from 'next/link'
import { usePathname } from 'next/navigation'
import posthog from 'posthog-js'
import { Suspense, useContext, useEffect, useRef, useState } from 'react'
import { isMobile } from 'react-device-detect'
import Joyride, { STATUS } from 'react-joyride'
import { toast } from 'sonner'
import { Editable } from '../../_components/EditableText'
import { type EditorFile, createUrl } from '../../actions'
import {
  PROJECT_ROOT,
  currentEditorFilesAtom,
  exploreProjectsOpenAtom,
  productTourDoneAtom,
  unsavedChangesAtom,
} from '../_atoms/atoms'
import { CodeMirrorEditor } from './CodeMirrorEditor'

import { GithubStars } from './GithubStars'
import { InitialTour, PostTestRunTour } from './Tour'
// import SettingsDialog, { ShowSettingsButton } from '@baml/playground-common/shared/SettingsDialog'

import FileViewer from './Tree/FileViewer'
// import { AppStateProvider } from '@baml/playground-common/shared/AppStateContext' // Import the AppStateProvider
// import { ViewSelector } from '@baml/playground-common/shared/Selectors'
import { useFeedbackWidget } from '@baml/playground-common/lib/feedback_widget'

const ProjectViewImpl = ({ project }: { project: BAMLProject }) => {
  useFeedbackWidget()
  // const setEditorFiles = useSetAtom(updateFileAtom)
  useKeybindingOverrides()
  // Tried to use url pathnames for this but nextjs hijacks the pathname state (even the window.location) so we have to manually track unsaved changes in the app.
  const [unsavedChanges, setUnsavedChanges] = useAtom(unsavedChangesAtom)

  useEffect(() => {
    if (project) {
      console.log('Updating files due: project', project.id)
      setUnsavedChanges(false)
      // setEditorFiles({
      //   reason: 'project_reload',
      //   replace_all: true,
      //   root_path: PROJECT_ROOT,
      //   files: project.files.map((f) => {
      //     return {
      //       name: f.path,
      //       content: f.content,
      //     }
      //   }),
      // })
      // TODO: @hellovai use this to set the test run output
      // project.testRunOutput
    }
  }, [project.id])
  const [projectName, setProjectName] = useState(project.name)
  const projectNameInputRef = useRef(null)
  const [description, setDescription] = useState(project.description)
  const descriptionInputRef = useRef(null)
  const setOpenExplorePanel = useSetAtom(exploreProjectsOpenAtom)

  return (
    // firefox wont apply the background color for some reason so we forcefully set it.
    <div className='flex relative flex-row w-full h-full bg-gray-800 main-panel overflow-x-clip overflow-y-clip'>
      {isMobile && (
        <div className='absolute bottom-0 left-0 right-0 text-zinc-900 font-semibold bg-zinc-400 border-t-zinc-600 border-t-[1px] w-full h-[100px] z-50 text-center p-8'>
          Visit PromptFiddle on Desktop to get the best experience
        </div>
      )}
      <ResizablePanelGroup className='w-full h-full overflow-clip' direction='horizontal'>
        {!isMobile && (
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

              {/* <ResizableHandle className='bg-white/10' />
              <ResizablePanel className='flex flex-col items-center pt-2 w-full tour-templates'></ResizablePanel> */}
            </ResizablePanelGroup>
          </ResizablePanel>
        )}

        <ResizableHandle className='bg-vscode-contrastActiveBorder border-vscode-contrastActiveBorder' />
        <ResizablePanel defaultSize={88}>
          <div className='flex-col w-full h-full font-sans bg-background dark:bg-vscode-panel-background'>
            <div className='flex flex-row items-center gap-x-12 border-b-[1px] border-vscode-panel-border min-h-[40px]'>
              <div className='flex flex-col items-center py-1 h-full whitespace-nowrap tour-title'>
                <Editable
                  text={projectName}
                  placeholder='Write a task name'
                  type='input'
                  childRef={projectNameInputRef}
                >
                  <input
                    className='px-2 text-lg border-none text-foreground'
                    type='text'
                    ref={projectNameInputRef}
                    name='task'
                    placeholder='Write a task name'
                    value={projectName}
                    onChange={(e) => setProjectName(e.target.value)}
                  />
                </Editable>
              </div>
              <div className='flex flex-row gap-x-2 items-center'>
                <ShareButton project={project} projectName={projectName} />
              </div>

              <div className='flex items-center justify-start h-full pt-0.5 '>
                <Button asChild variant={'ghost'} className='gap-x-1 py-1 h-full hover:bg-indigo-600'>
                  <Link
                    href='https://boundaryml.com'
                    target='_blank'
                    className='text-sm hover:text-foreground text-foreground'
                  >
                    What is BAML?
                  </Link>
                </Button>
              </div>
              {project.id !== 'all-projects' && project.id !== null ? (
                <div className='flex flex-col justify-center items-center h-full'>
                  <Link
                    href={`/all-projects`}
                    target='_blank'
                    className='flex flex-row gap-x-2 items-center px-2 py-1 text-sm whitespace-pre-wrap bg-indigo-600 rounded-sm hover:bg-indigo-500 h-fit text-vscode-button-foreground'
                  >
                    <Compass size={16} strokeWidth={2} />
                    <span className='whitespace-nowrap'>Explore Examples</span>
                  </Link>
                </div>
              ) : null}
              <div className='flex flex-col justify-center items-center h-full'>
                <Link
                  href={`/new-project`}
                  target='_blank'
                  className='flex flex-row gap-x-2 items-center px-2 py-1 text-sm whitespace-pre-wrap bg-indigo-600 rounded-sm hover:bg-indigo-500 h-fit text-vscode-button-foreground'
                >
                  <File size={16} strokeWidth={2} />
                  <span className='whitespace-nowrap'>New project</span>
                </Link>
              </div>
              {unsavedChanges ? (
                <div className='flex flex-row items-center whitespace-nowrap text-muted-foreground'>
                  <Badge variant='outline' className='gap-x-2 font-light text-yellow-400'>
                    <AlertTriangleIcon size={14} />
                    <span>Unsaved changes</span>
                  </Badge>
                </div>
              ) : (
                <></>
              )}

              <div className='flex flex-row gap-x-8 justify-end items-center pr-4 w-full'>
                <div className='flex h-full'>
                  <Link
                    href='https://discord.gg/BTNBeXGuaS'
                    className='pt-0 h-full w-fit text-zinc-300 hover:text-zinc-50'
                  >
                    <div className='flex flex-row gap-x-4 items-center text-sm'>
                      <Image
                        src='/discord-icon.svg'
                        className='opacity-60 hover:opacity-100'
                        width={24}
                        height={24}
                        alt='Discord'
                      />
                    </div>
                  </Link>
                </div>
                <div className='flex h-full'>
                  <Link
                    href='https://docs.boundaryml.com/guide/installation-editors/vs-code-extension'
                    className='pt-0 h-full w-fit text-zinc-300 hover:text-zinc-50'
                  >
                    <div className='flex flex-row gap-x-4 items-center text-xs grayscale 2xl:text-sm hover:grayscale-0'>
                      <Image src='/vscode_logo.svg' width={18} height={18} alt='VSCode extension' />
                    </div>
                  </Link>
                </div>
                <div className='flex h-full'>
                  <GithubStars />
                </div>
              </div>
            </div>

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
                    {/* <CodeMirrorEditor project={project} /> */}
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
    </div>
  )
}

export const ProjectView = ({ project }: { project: BAMLProject }) => {
  return (
    <>
      <ProjectViewImpl project={project} />
    </>
  )
}

const ShareButton = ({ project, projectName }: { project: BAMLProject; projectName: string }) => {
  const [loading, setLoading] = useState(false)
  const editorFiles = useAtomValue(currentEditorFilesAtom)

  const pathname = usePathname()
  const [unsavedChanges, setUnsavedChanges] = useAtom(unsavedChangesAtom)

  return (
    <Button
      variant={'default'}
      className='gap-x-1 py-1 h-full whitespace-nowrap bg-transparent shadow-md text-vscode-button-foreground hover:bg-indigo-600 w-fit'
      disabled={loading}
      onClick={async () => {
        setLoading(true)
        try {
          let urlId = pathname?.split('/')[1]
          if (!urlId) {
            urlId = await createUrl({
              ...project,
              name: projectName,
              files: editorFiles,
              // TODO: @hellovai use runTestOutput
              testRunOutput: undefined,
            })

            posthog.capture('share_url', { id: urlId })

            const newUrl = `${window.location.origin}/${urlId}`
            window.history.replaceState({ ...window.history.state, as: newUrl, url: newUrl }, '', newUrl)
            setUnsavedChanges(false)
          }

          navigator.clipboard.writeText(`${window.location.origin}/${urlId}`)

          toast('URL copied to clipboard')
        } catch (e) {
          posthog.capture('share_url_failed', { error: JSON.stringify(e) })
          toast('Failed to generate URL')
          console.error(e)
        } finally {
          setLoading(false)
        }
      }}
    >
      {unsavedChanges ? <GitForkIcon size={14} /> : <LinkIcon size={14} />}
      <span>{unsavedChanges ? 'Fork & Share' : 'Share'}</span>
    </Button>
  )
}

const DummyHydrate = ({ files }: { files: EditorFile[] }) => {
  useHydrateAtoms([[currentEditorFilesAtom as any, files]]) // any cause sessionStorage screws types up somehow
  return <></>
}

// import dynamic from 'next/dynamic'
// const EventListener = dynamic(() => import('@baml/playground-common/baml_wasm_web/EventListener').then(mod => mod.EventListener), { ssr: false })

const PlaygroundView = () => {
  return (
    <>
      {/* <AppStateProvider> */}
        <CustomErrorBoundary message='Error loading playground'>
          <Suspense fallback={<div>Loading...</div>}>
            {/* <EventListener> */}
              {/* <SettingsDialog /> */}
              <div className='flex relative flex-col gap-2 pr-0 w-full'>
                <div className='flex relative flex-row gap-2'>
                  <div className='flex flex-row gap-2 justify-start items-start pr-1 grow'>
                    {/* <ViewSelector /> */}
                  </div>
                  <div className='flex relative flex-row gap-2 justify-end items-center pr-1 grow'></div>
                </div>
                {/* <Separator className="bg-vscode-textSeparator-foreground" /> */}
                {/* <FunctionPanel /> */}
              </div>
              {/* <InitialTour /> */}
              {/* <PostTestRunTour /> */}
            {/* </EventListener> */}
          </Suspense>
        </CustomErrorBoundary>
      {/* </AppStateProvider> */}
    </>
  )
}

export default ProjectView
