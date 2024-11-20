/* eslint-disable @typescript-eslint/no-explicit-any */
'use client'

import { useEffect, useState } from 'react'
import { ChevronRight, File } from 'lucide-react'
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@/components/ui/collapsible'
import * as React from 'react'
import { getItems } from '@/app/components/get-items'

export class SelectionEvent extends Event {
  public readonly selection: any

  constructor(selection: any) {
    super(SelectionEvent.NAME)
    this.selection = selection
  }

  public static readonly NAME = 'item-selection'
}

export class RefreshSelectionEvent extends Event {
  public readonly selection: any

  constructor(selection: any) {
    super(RefreshSelectionEvent.NAME)
    this.selection = selection
  }

  public static readonly NAME = 'refresh-selection'
}


export default function Tree({ item }: { item: any }) {

  const [items, setItems] = useState(item.items || [])

  function onSelected() {
    const event = new SelectionEvent(item)
    window.dispatchEvent(event)
  }

  async function refresh() {
    setItems(await getItems(item.id))
  }

  useEffect(() => {
    window?.addEventListener(RefreshSelectionEvent.NAME, refresh)
    return () => {
      window?.removeEventListener(RefreshSelectionEvent.NAME, refresh)
    }
  })

  if (item.__typename === 'Metadata') {
    return (
      <div
        className="flex min-w-40 ps-4 pt-1 pb-1 gap-2 cursor-pointer data-[active=true]:bg-transparent text-sm"
        onClick={() => onSelected()}
      >
        <File className="h-4 w-4 transition-transform place-self-center"/>
        {item.name}
      </div>
    )
  }

  async function onOpenChange(open: boolean) {
    if (open) {
      await refresh()
    }
  }

  return (
    <Collapsible
      className="group/collapsible [&[data-state=open]>div>svg:first-child]:rotate-90"
      defaultOpen={item.name === 'Root'}
      onOpenChange={onOpenChange}
    >
      <CollapsibleTrigger asChild>
        <div onClick={() => onSelected()} className="flex min-w-40 gap-2 cursor-pointer text-sm pb-1 pt-1">
          <ChevronRight className="h-4 w-4 transition-transform place-self-center"/>
          {item.name}
        </div>
      </CollapsibleTrigger>
      <CollapsibleContent className="ps-2">
        {items.map((subItem: any, index: any) => (
          <Tree key={index} item={subItem} />
        ))}
      </CollapsibleContent>
    </Collapsible>
  )
}
