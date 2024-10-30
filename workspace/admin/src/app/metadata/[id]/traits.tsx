'use client'

import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Label } from '@/components/ui/label'
import React from 'react'

interface TraitsProps extends React.HTMLAttributes<HTMLDivElement> {
  id: string
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  traits: any[]
  current: string[]
}

export function Traits({ id, traits, current } : TraitsProps) {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  function onChange(value: any) {
    document.location = '/metadata/' + id + '?add-trait=' + value
  }
  return (
    <>
      <Label htmlFor="traits">Trait to Add</Label>
      <Select onValueChange={onChange}>
        <SelectTrigger id="status" aria-label="Select trait to add">
          <SelectValue placeholder="Select trait to add"/>
        </SelectTrigger>
        <SelectContent>
          {traits.filter((t) => !current.includes(t.id)).map((trait) =>
            <SelectItem key={trait.id} value={trait.id}>{trait.name || trait.id}</SelectItem>)
          }
        </SelectContent>
      </Select>
    </>
  )
}
