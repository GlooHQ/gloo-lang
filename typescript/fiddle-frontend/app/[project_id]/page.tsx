import type { BAMLProject } from '@/lib/exampleProjects'
import { loadProject } from '@/lib/loadProject'
import type { Metadata, ResolvingMetadata } from 'next'
import dynamic from 'next/dynamic'
// const ProjectView = dynamic(() => import('./_components/ProjectView'), { ssr: true })
import ProjectView from './_components/ProjectView'

type Props = {
  params: { project_id: string }
  searchParams: { [key: string]: string | string[] | undefined }
}
export async function generateMetadata({ params, searchParams }: Props, parent: ResolvingMetadata): Promise<Metadata> {
  // read route params
  try {
    const project = await loadProject(Promise.resolve(params))
    return {
      title: `${project.name} — Prompt Fiddle`,
      description: project.description,
    }
  } catch (e) {
    console.log('Error generating metadata', e)
    return {
      title: 'Prompt Fiddle',
      description: 'An LLM prompt playground for structured prompting',
    }
  }
}

type SearchParams = {
  id: string
}

export default async function Home({
  searchParams,
  params,
}: {
  searchParams: SearchParams
  params: { project_id: string }
}) {
  const data: BAMLProject = await loadProject(Promise.resolve(params))
  // console.log(data)
  return (
    <main className='flex flex-col justify-between items-center min-h-screen font-sans'>
      <div className='w-screen h-screen dark:bg-black'>
        <ProjectView project={data} />
      </div>
    </main>
  )
}
