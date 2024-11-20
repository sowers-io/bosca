'use client'

import {
  Menubar,
  MenubarContent,
  MenubarItem,
  MenubarMenu,
  MenubarSeparator,
  MenubarShortcut,
  MenubarSub,
  MenubarSubContent,
  MenubarSubTrigger,
  MenubarTrigger,
} from '@/components/ui/menubar'
import React, { useEffect, useState } from 'react'
import { RefreshSelectionEvent, SelectionEvent } from '@/app/components/app-sidebar-tree'
import { addNewCollection, addNewMetadata } from '@/app/components/menu-graphql'

interface MenuProps extends React.HTMLAttributes<HTMLDivElement> {
  parent?: string | undefined
}

export function Menu({ parent }: MenuProps) {
  if (!parent) parent = '00000000-0000-0000-0000-000000000000'

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const [selection, setSelection] = useState<any | undefined>(undefined)
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const [selectedCollection, setSelectedCollection] = useState<any | undefined>(undefined)

  function onSelection(e: SelectionEvent) {
    setSelection(e.selection)
    if (e.selection.__typename === 'Collection') {
      setSelectedCollection(e.selection)
    }
  }

  function refreshTree() {
    window.dispatchEvent(new RefreshSelectionEvent(selectedCollection))
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

  return (
    <Menubar className="rounded-none border-b border-none px-2 lg:px-4">
      <MenubarMenu>
        <MenubarTrigger className="font-bold">Bosca</MenubarTrigger>
        <MenubarContent>
          <MenubarItem disabled={true}>About Bosca</MenubarItem>
          <MenubarSeparator/>
          <MenubarItem disabled={true}>
            Preferences...
          </MenubarItem>
        </MenubarContent>
      </MenubarMenu>
      <MenubarMenu>
        <MenubarTrigger className="relative">File</MenubarTrigger>
        <MenubarContent>
          <MenubarSub>
            <MenubarSubTrigger>New</MenubarSubTrigger>
            <MenubarSubContent className="w-[230px]">
              <MenubarItem disabled={!selectedCollection}>
                <button onClick={async () => {
                  await addNewCollection('New Collection', selectedCollection.id)
                  refreshTree()
                }}>Collection
                </button>
              </MenubarItem>
              <MenubarItem disabled={!selectedCollection}>
                <button onClick={async () => {
                  await addNewMetadata('New Metadata', selectedCollection.id)
                  refreshTree()
                }}>Metadata
                </button>
              </MenubarItem>
            </MenubarSubContent>
          </MenubarSub>
          <MenubarSeparator/>
          <MenubarSub>
            <MenubarSubTrigger>Library</MenubarSubTrigger>
            <MenubarSubContent>
              <MenubarItem disabled={true}>Organize Library...</MenubarItem>
              <MenubarItem disabled={true}>Export Library...</MenubarItem>
              <MenubarSeparator/>
              <MenubarItem disabled={true}>Import Library...</MenubarItem>
            </MenubarSubContent>
          </MenubarSub>
        </MenubarContent>
      </MenubarMenu>
      <MenubarMenu>
        <MenubarTrigger>Edit</MenubarTrigger>
        <MenubarContent>
          <MenubarItem disabled>
            Undo <MenubarShortcut>⌘Z</MenubarShortcut>
          </MenubarItem>
          <MenubarItem disabled>
            Redo <MenubarShortcut>⇧⌘Z</MenubarShortcut>
          </MenubarItem>
          <MenubarSeparator/>
          <MenubarItem disabled>
            Cut <MenubarShortcut>⌘X</MenubarShortcut>
          </MenubarItem>
          <MenubarItem disabled>
            Copy <MenubarShortcut>⌘C</MenubarShortcut>
          </MenubarItem>
          <MenubarItem disabled>
            Paste <MenubarShortcut>⌘V</MenubarShortcut>
          </MenubarItem>
          <MenubarSeparator/>
          <MenubarItem>
            Select All <MenubarShortcut>⌘A</MenubarShortcut>
          </MenubarItem>
          <MenubarItem disabled>
            Deselect All <MenubarShortcut>⇧⌘A</MenubarShortcut>
          </MenubarItem>
        </MenubarContent>
      </MenubarMenu>
      <MenubarMenu>
        <MenubarTrigger className="hidden md:block">Account</MenubarTrigger>
        <MenubarContent forceMount>
          <MenubarItem disabled={true} inset>Manage Account...</MenubarItem>
        </MenubarContent>
      </MenubarMenu>
    </Menubar>
  )
}