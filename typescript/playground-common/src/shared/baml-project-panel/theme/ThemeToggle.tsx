'use client'

import { Moon, Sun } from 'lucide-react'
import { useTheme } from 'next-themes'
import { Button } from '~/components/ui/button'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '~/components/ui/dropdown-menu'

export function ThemeToggle() {
  const { setTheme, theme } = useTheme()

  return (
    <Button
      variant='outline'
      size='icon'
      className='relative p-0 px-2 py-2 w-6 h-6'
      onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
    >
      <Sun className='absolute w-3 h-3 opacity-100 transition-all duration-300 scale-100 rotate-0 dark:scale-0 dark:opacity-0' />
      <Moon className='absolute w-3 h-3 opacity-0 transition-all duration-300 scale-0 rotate-90 dark:rotate-0 dark:scale-100 dark:opacity-100' />
      <span className='sr-only'>Toggle theme</span>
    </Button>
  )
}
