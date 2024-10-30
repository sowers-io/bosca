'use client'

import { Label } from '@/components/ui/label'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import React from 'react'

interface StatesProps extends React.HTMLAttributes<HTMLDivElement> {
  id: string
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  states: any[]
  current: string
  pending: string | null
}

export function States({ id, states, current, pending }: StatesProps) {
  return (
    <>
      <Label htmlFor="status">State</Label>
      {pending ? 'Pending: ' + states.filter((s) => s.id === pending)[0].name :
        <Select value={current}
          onValueChange={(value) => document.location = '/collections/' + id + '?set-state=' + value}>
          <SelectTrigger id="status" aria-label="Select status">
            <SelectValue placeholder="Select state"/>
          </SelectTrigger>
          <SelectContent>
            {states.map((state) =>
              <SelectItem key={state.id} value={state.id}>{state.name}</SelectItem>)}
          </SelectContent>
        </Select>
      }
    </>
  )
}