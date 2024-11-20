'use client'

import { useEffect, useState } from 'react'
import { SelectionEvent } from '@/app/components/app-sidebar-tree'
import { Badge } from '@/components/ui/badge'

export function Selection() {

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const [selection, setSelection] = useState<any | undefined>(undefined)

  function onSelection(e: SelectionEvent) {
    setSelection(e.selection)
  }

  useEffect(() => {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-expect-error
    window?.addEventListener('item-selection', onSelection)
    return () => {
      // eslint-disable-next-line @typescript-eslint/ban-ts-comment
      // @ts-expect-error
      window?.removeEventListener('item-selection', onSelection)
    }
  })

  return !selection ? (
    <div className="grid grid-cols-1 w-full h-full text-center">
      {/* eslint-disable-next-line @next/next/no-img-element */}
      <img src="/bosca.svg" className="w-96 h-96 m-auto" alt="Bosca" />
      <h1 className="text-4xl">Bosca</h1>
      <div>Press <Badge variant="outline">Ctrl + Space</Badge> to get started.</div>
    </div>
  ) : (
    <div className="flex flex-1 flex-col gap-4 p-4">
      <h1 className={'font-bold'}>{selection.name}</h1>
      <h2>{selection.description ?? ''}</h2>
    </div>
  )
}