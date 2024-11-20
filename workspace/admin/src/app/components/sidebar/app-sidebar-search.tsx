'use client'

import { Search } from 'lucide-react'
import * as React from 'react'
import { Input } from '@/components/ui/input'

export default function SearchForm() {
  return (
    <div className="flex relative m-2">
      <Search className="pointer-events-none absolute left-2 top-1/2 size-4 -translate-y-1/2 select-none opacity-50" />
      <Input
        id="search"
        placeholder="Search..."
        className="pl-8 border-0"
        onClick={() => document.dispatchEvent(new KeyboardEvent('keydown', { key: ' ', ctrlKey: true }))}
      />
    </div>
  )
}