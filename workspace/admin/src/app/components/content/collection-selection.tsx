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

interface CollectionSelection {
  collection: any
}

export function CollectionSelection({ collection }: CollectionSelection) {
  return (
    <Card className="w-[350px]">
      <CardHeader>
        <CardTitle>Manage Collection</CardTitle>
      </CardHeader>
      <CardContent>
        <form>
          <div className="grid w-full items-center gap-4">
            <div className="flex flex-col space-y-1.5">
              <Label htmlFor="name">Name</Label>
              <Input id="name" placeholder="Name" defaultValue={collection.name} />
            </div>
          </div>
        </form>
      </CardContent>
      <CardFooter className="flex justify-between">
        <Button>Save</Button>
      </CardFooter>
    </Card>
  )
}
