'use client'

import { useEffect, useState } from 'react'
import { ClearSelectionEvent, SelectionEvent } from '@/app/components/sidebar/app-sidebar-tree'
import { Badge } from '@/components/ui/badge'
import { MetadataSelection } from '@/app/components/content/metadata-selection'
import { CollectionSelection } from '@/app/components/content/collection-selection'

export function Selection() {

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const [selection, setSelection] = useState<any | undefined>(undefined)

  function onSelection(e: SelectionEvent) {
    setSelection(e.selection)
  }

  function onClearSelection() {
    setSelection(undefined)
  }

  useEffect(() => {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-expect-error
    window?.addEventListener(SelectionEvent.NAME, onSelection)
    window?.addEventListener(ClearSelectionEvent.NAME, onClearSelection)
    return () => {
      // eslint-disable-next-line @typescript-eslint/ban-ts-comment
      // @ts-expect-error
      window?.removeEventListener(SelectionEvent.NAME, onSelection)
      // eslint-disable-next-line @typescript-eslint/ban-ts-comment
      // @ts-expect-error
      window?.removeEventListener(ClearSelectionEvent.NAME, onSelection)
    }
  })

  return !selection ? (
    <div className="grid grid-cols-1 w-full h-full text-center">
      {/* eslint-disable-next-line @next/next/no-img-element */}
      <img src="/bosca.svg" className="w-96 h-96 m-auto" alt="Bosca"/>
      <h1 className="text-4xl">Bosca</h1>
      <div>Press <Badge variant="outline">Ctrl + Space</Badge> to get started.</div>
    </div>
  ) : (
    <div className="flex flex-1 flex-col gap-4 p-4">
      {
        selection.__typename === 'Collection' ?
          <CollectionSelection collection={selection}/> :
          <MetadataSelection metadata={selection}/>
      }
    </div>
  )
}