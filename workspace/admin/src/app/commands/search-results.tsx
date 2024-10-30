'use client'

import React from 'react'
import { CommandGroup, CommandItem } from '@/components/ui/command'
import { SearchDocument } from '@/app/commands/models'

export function SearchResults({ heading, documents, onSelect }: {
  heading: string
  documents: SearchDocument[]
  onSelect: (document: SearchDocument) => void
}) {
  return (
    <CommandGroup heading={heading}>
      {documents.map((doc) => {
        if (doc?.collection) {
          return (
            <CommandItem key={doc.collection.id} onSelect={(_) => onSelect(doc)}>
              {doc.collection.name}
            </CommandItem>
          )
        }
        if (doc?.metadata) {
          return (
            <CommandItem key={doc.metadata.id} onSelect={(_) => onSelect(doc)}>
              {doc.metadata.name}
            </CommandItem>
          )
        }
        return <></>
      })}
      {documents.length === 0 ? <CommandItem>No Results</CommandItem> : <></>}
    </CommandGroup>
  )
}