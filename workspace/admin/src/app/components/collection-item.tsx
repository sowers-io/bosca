import { cn } from '@/lib/utils'
import React from 'react'

export interface Collection {
  id: string,
  name: string
  description: string | null
}

interface CollectionItemProps extends React.HTMLAttributes<HTMLDivElement> {
  collection: Collection
}

export function CollectionItem({ collection, className, ...props }: CollectionItemProps) {
  return (
    <div className={cn('space-y-3', className)} {...props}>
      <a className="space-y-1 text-sm" href={'/collections/' + collection.id}>
        <h3 className="font-medium leading-none">{collection.name}</h3>
        <p className="text-xs text-muted-foreground">{collection.description || ''}</p>
      </a>
    </div>
  )
}