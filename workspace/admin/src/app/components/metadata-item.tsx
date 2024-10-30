import { cn } from '@/lib/utils'
import Link from 'next/link'

export interface Metadata {
  id: string,
  name: string
  type: string
}

interface CollectionItemProps extends React.HTMLAttributes<HTMLDivElement> {
  metadata: Metadata
  parent: string
}

export function MetadataItem({ parent, metadata, className, ...props }: CollectionItemProps) {
  return (
    <div className={cn('space-y-3', className)} {...props}>
      <Link className="space-y-1 text-sm" href={'/metadata/' + metadata.id + '?parent=' + parent}>
        <h3 className="font-medium leading-none">{metadata.name}</h3>
        <p className="text-xs text-muted-foreground">{metadata.type || ''}</p>
      </Link>
    </div>
  )
}