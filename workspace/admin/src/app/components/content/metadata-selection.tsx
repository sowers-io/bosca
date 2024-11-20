/* eslint-disable @typescript-eslint/no-explicit-any */
import * as React from 'react'

import { Button } from '@/components/ui/button'
import {
  Card,
  CardContent,
  CardFooter, CardHeader, CardTitle,
} from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { States } from '@/app/components/content/states'
import { getWorkflowData } from '@/app/components/get-items'
import { useEffect, useState } from 'react'

interface MetadataSelection {
  metadata: any
}

export function MetadataSelection({ metadata }: MetadataSelection) {
  const [states, setStates] = useState([])

  useEffect(() => {
    async function initWorkflowData() {
      const workflows = await getWorkflowData()
      setStates(workflows.states.all)
    }
    initWorkflowData().then(_ => {})
  })

  return (
    <>
      <Card className="w-[350px]">
        <CardHeader>
          <CardTitle>Metadata Details</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid w-full items-center gap-4">
            <div className="flex flex-col space-y-1.5">
              <Label htmlFor="name">Name</Label>
              <Input id="name" placeholder="Name" defaultValue={metadata.name} />
            </div>
          </div>
        </CardContent>
        <CardFooter className="flex justify-between">
          <Button>Save</Button>
        </CardFooter>
      </Card>
      <Card className="w-[350px]">
        <CardHeader>
          <CardTitle>Workflow State</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid w-full items-center gap-4">
            <div className="flex flex-col space-y-1.5">
              <States states={states} current={metadata.workflow.state} pending={metadata.workflow.pending} onSelected={() => {}} />
            </div>
          </div>
        </CardContent>
        <CardFooter className="flex justify-between">
          <Button>Save</Button>
        </CardFooter>
      </Card>
    </>
  )
}
