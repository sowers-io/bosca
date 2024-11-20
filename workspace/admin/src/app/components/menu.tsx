'use client'

import {
  Menubar,
  MenubarContent,
  MenubarItem,
  MenubarMenu,
  MenubarSeparator,
  MenubarSub,
  MenubarSubContent,
  MenubarSubTrigger,
  MenubarTrigger,
} from '@/components/ui/menubar'
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from '@/components/ui/alert-dialog'
import React, { useEffect, useState } from 'react'
import { ClearSelectionEvent, RefreshSelectionEvent, SelectionEvent } from '@/app/components/sidebar/app-sidebar-tree'
import { addNewCollection, addNewMetadata, deleteCollection, deleteMetadata } from '@/app/components/menu-graphql'

interface MenuProps extends React.HTMLAttributes<HTMLDivElement> {
  parent?: string | undefined
}

export function Menu({ parent }: MenuProps) {
  if (!parent) parent = '00000000-0000-0000-0000-000000000000'

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const [selection, setSelection] = useState<any | undefined>(undefined)
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const [selectedCollection, setSelectedCollection] = useState<any | undefined>(undefined)
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const [confirmDelete, setConfirmDelete] = useState<any | undefined>(undefined)

  function onSelection(e: SelectionEvent) {
    setSelection(e.selection)
    if (e.selection?.__typename === 'Collection') {
      setSelectedCollection(e.selection)
    }
  }

  function onAdvancedEdit() {
    if (selection?.__typename === 'Collection') {
      document.location = '/collection/' + selection.id
    } else if (selection?.__typename === 'Metadata') {
      document.location = '/metadata/' + selection.id
    }
  }

  async function onDelete() {
    if (confirmDelete.__typename === 'Collection') {
      await deleteCollection(confirmDelete.id)
    } else if (confirmDelete.__typename === 'Metadata') {
      await deleteMetadata(confirmDelete.id)
    }
    refreshTree()
    setSelection(undefined)
    setSelectedCollection(undefined)
    setConfirmDelete(undefined)
    window.dispatchEvent(new ClearSelectionEvent())
  }

  function refreshTree() {
    window.dispatchEvent(new RefreshSelectionEvent(selectedCollection))
  }

  useEffect(() => {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-expect-error
    window?.addEventListener(SelectionEvent.NAME, onSelection)
    return () => {
      // eslint-disable-next-line @typescript-eslint/ban-ts-comment
      // @ts-expect-error
      window?.removeEventListener(SelectionEvent.NAME, onSelection)
    }
  })

  return (
    <>
      <Menubar>
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
                  }}>{selectedCollection ? 'Collection in ' + selectedCollection.name : 'Collection'}
                  </button>
                </MenubarItem>
                <MenubarItem disabled={!selectedCollection}>
                  <button onClick={async () => {
                    await addNewMetadata('New Metadata', selectedCollection.id)
                    refreshTree()
                  }}>{selectedCollection ? 'Metadata in ' + selectedCollection.name : 'Metadata'}
                  </button>
                </MenubarItem>
              </MenubarSubContent>
            </MenubarSub>
            <MenubarSeparator/>
            <MenubarItem disabled={!selection} onClick={() => setConfirmDelete(selection)}>Delete</MenubarItem>
          </MenubarContent>
        </MenubarMenu>
        <MenubarMenu>
          <MenubarTrigger className="hidden md:block">Edit</MenubarTrigger>
          <MenubarContent>
            <MenubarItem disabled={!selection} onClick={onAdvancedEdit}>Advanced</MenubarItem>
          </MenubarContent>
        </MenubarMenu>
        <MenubarMenu>
          <MenubarTrigger className="hidden md:block">Account</MenubarTrigger>
          <MenubarContent forceMount>
            <MenubarItem disabled={true} inset>Manage Account...</MenubarItem>
          </MenubarContent>
        </MenubarMenu>
      </Menubar>
      <AlertDialog open={confirmDelete !== undefined}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete {confirmDelete?.name}.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel onClick={() => setConfirmDelete(undefined)}>Cancel</AlertDialogCancel>
            <AlertDialogAction onClick={onDelete}>Delete</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  )
}