'use client'

import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import React from 'react'

interface Workflows extends React.HTMLAttributes<HTMLDivElement> {
  id: string
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  workflows: any[]
}

export function Workflows({ id, workflows } : Workflows) {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  function onChange(value: any) {
    document.location = '/metadata/' + id + '?workflow=' + value
  }
  return (
    <>
      <Select onValueChange={onChange}>
        <SelectTrigger id="status" aria-label="Select workflow to run">
          <SelectValue placeholder="Select workflow to run"/>
        </SelectTrigger>
        <SelectContent>
          {workflows.map((workflow) =>
            <SelectItem key={workflow.id} value={workflow.id}>{workflow.name || workflow.id}</SelectItem>)
          }
        </SelectContent>
      </Select>
    </>
  )
}
