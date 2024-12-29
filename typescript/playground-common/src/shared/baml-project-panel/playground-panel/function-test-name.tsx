import { ChevronRight, FlaskConical, FunctionSquare } from 'lucide-react'

interface FunctionTestNameProps {
  functionName: string
  testName: string
  selected?: boolean
}
export const FunctionTestName: React.FC<FunctionTestNameProps> = ({ functionName, testName, selected }) => {
  return (
    <div className={`flex w-full items-center space-x-1 text-xs ${selected ? '' : 'text-muted-foreground'}`}>
      <div className='flex items-center'>
        <FunctionSquare className='mr-1 h-3 w-3' />
        {functionName}
      </div>
      <ChevronRight className='h-3 w-3' />
      <div className='flex items-center'>
        <FlaskConical className='mr-1 h-3 w-3' />
        {testName}
      </div>
    </div>
  )
}
