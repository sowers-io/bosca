'use client'

import { Command, CommandInput, CommandList } from '@/components/ui/command'
import React from 'react'
import { onCommandValueChange } from '@/app/actions'
import { Dialog, DialogContent } from '@/components/ui/dialog'
import { ActionHandler, ActionState, CommandMenuAction, CurrentItem, SearchDocument } from '@/app/commands/models'
import { Loading } from '@/app/commands/loading'
import { SearchResults } from '@/app/commands/search-results'
import { Suggestions } from '@/app/commands/suggestions'

export { onCommandValueChange } from '@/app/actions'

interface CommandMenuProps extends React.HTMLAttributes<HTMLDivElement> {
  current: CurrentItem | null,
  actions: CommandMenuAction[]
}

let change = 0
let changing = 0

export function CommandMenu({ current }: CommandMenuProps) {
  const [loading, setLoading] = React.useState(false)
  const [open, setOpen] = React.useState(false)
  const [filter, setFilter] = React.useState<string | null>(null)
  const [actionState, setActionState] = React.useState(ActionState.DEFAULT)
  const [documents, setDocuments] = React.useState<SearchDocument[]>([])

  const handler: ActionHandler = {
    setAddMetadataToCollection() {
      setActionState(ActionState.ADD_METADATA_TO_COLLECTION)
      setFilter('_type = "collection"')
      setDocuments([])
    },
    setAddCollectionToCollection() {
      setActionState(ActionState.ADD_COLLECTION_TO_COLLECTION)
      setFilter('_type = "collection"')
      setDocuments([])
    },
    setAddMetadataRelationship() {
      setActionState(ActionState.ADD_METADATA_RELATIONSHIP)
      setFilter('_type = "metadata"')
      setDocuments([])
    },
    openCollection(id: string) {
      document.location = '/collections/' + id
      setOpen(false)
    },
    createNewMetadata(parent: string) {
      document.location = '/metadata/add?parent=' + parent
      setOpen(false)
    },
    createNewCollection(parent: string) {
      document.location = '/collections/add?parent=' + parent
      setOpen(false)
    },
  }

  React.useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === ' ' && e.ctrlKey) {
        e.preventDefault()
        setOpen((open) => !open)
      }
    }
    document.addEventListener('keydown', down)
    return () => document.removeEventListener('keydown', down)
  }, [])

  async function onValueChange(value: string) {
    if (value === '') {
      setDocuments([])
      return
    }
    changing++
    if (changing == 1) {
      setLoading(true)
    }
    try {
      change++
      const currentChange = change
      const result = await onCommandValueChange(value, filter)
      if (currentChange === change) {
        const documents = result.documents.filter((d: SearchDocument) => {
          if (!current) return true
          if (d.collection?.id === current?.id) return false
          return d.metadata?.id !== current?.id
        })
        setDocuments(documents)
      }
    } catch (_) {
      setDocuments([])
    } finally {
      changing--
      if (changing == 0) {
        setLoading(false)
      }
    }
  }

  let list = <></>
  switch (actionState) {
    case ActionState.ADD_COLLECTION_TO_COLLECTION:
    case ActionState.ADD_METADATA_TO_COLLECTION:
      list = <SearchResults heading={'Select Collection'} documents={documents} onSelect={(result) => {
        if (current?.collection) {
          document.location = '/collections/add-to-collection?parent=' + result.collection?.id + '&id=' + current?.id
        } else {
          document.location = '/metadata/add-to-collection?parent=' + result.collection?.id + '&id=' + current?.id
        }
        setOpen(false)
      }} />
      break
    case ActionState.ADD_METADATA_RELATIONSHIP:
      list = <SearchResults heading={'Select Metadata'} documents={documents} onSelect={(result) => {
        document.location = '/metadata/' + current?.id + '?add-relationship=' + result.metadata?.id
        setOpen(false)
      }} />
      break
    default:
      list = <>
        <SearchResults heading={'Search Results'} documents={documents} onSelect={(result) => {
          if (result.collection) {
            document.location = '/collections/' + result.collection.id
            setOpen(false)
          }
          if (result.metadata) {
            document.location = '/metadata/' + result.metadata.id
            setOpen(false)
          }
        }} />
        <Suggestions current={current} handler={handler} actionState={actionState} />
      </>
  }

  function onOpenChange(open: boolean) {
    setOpen(open)
    if (!open) {
      setTimeout(() => {
        setFilter(null)
        setDocuments([])
        setActionState(ActionState.DEFAULT)
      }, 500)
    }
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="overflow-hidden p-0">
        <Command
          shouldFilter={false}
          className="[&_[cmdk-group-heading]]:px-2 [&_[cmdk-group-heading]]:font-medium [&_[cmdk-group-heading]]:text-muted-foreground [&_[cmdk-group]:not([hidden])_~[cmdk-group]]:pt-0 [&_[cmdk-group]]:px-2 [&_[cmdk-input-wrapper]_svg]:h-5 [&_[cmdk-input-wrapper]_svg]:w-5 [&_[cmdk-input]]:h-12 [&_[cmdk-item]]:px-2 [&_[cmdk-item]]:py-3 [&_[cmdk-item]_svg]:h-5 [&_[cmdk-item]_svg]:w-5">
          <CommandInput placeholder="Search..." onValueChange={onValueChange}/>
          <CommandList>
            {loading ? <Loading /> : <></>}
            {list}
          </CommandList>
        </Command>
      </DialogContent>
    </Dialog>
  )
}
