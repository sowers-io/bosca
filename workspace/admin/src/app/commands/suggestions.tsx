import { CommandGroup, CommandItem } from '@/components/ui/command'
import { ActionHandler, ActionState, CurrentItem } from '@/app/commands/models'
import React from 'react'

interface SuggestionsProps extends React.HTMLAttributes<HTMLDivElement> {
  current: CurrentItem | null
  handler: ActionHandler
  actionState: ActionState
}

const rootCollectionId = '00000000-0000-0000-0000-000000000000'

export function Suggestions({ current, handler, actionState }: SuggestionsProps) {
  const parent = current ? (current?.collection ? current.id : (current.item.parentCollections[0]?.id || rootCollectionId)) : rootCollectionId
  const parentName = current ? (current?.collection ? current.item.name : (current.item.parentCollections[0]?.name || 'Root')) : 'Root'

  let list = <></>
  switch (actionState) {
    case ActionState.DEFAULT:
      if (current) {
        if (current.collection) {
          list = <CommandItem onSelect={handler.setAddCollectionToCollection}>Add {current.item.name} to a collection</CommandItem>
        } else {
          list = <>
            <CommandItem onSelect={handler.setAddMetadataToCollection}>Add {current.item.name} to a collection</CommandItem>
            <CommandItem onSelect={handler.setAddMetadataRelationship}>Add a relationship for {current.item.name}</CommandItem>
          </>
        }
      }
      break
    default:
  }

  return (
    <CommandGroup heading="Suggestions">
      {list}
      {
        !current ?
          <CommandItem onSelect={(_) => handler.openCollection(rootCollectionId)}>Open Root Collection</CommandItem> :
          <></>
      }
      <CommandItem onSelect={(_) => handler.createNewMetadata(parent)}>Create New Metadata in {parentName}</CommandItem>
      <CommandItem onSelect={(_) => handler.createNewCollection(parent)}>Create New Collection in {parentName}</CommandItem>
    </CommandGroup>
  )
}