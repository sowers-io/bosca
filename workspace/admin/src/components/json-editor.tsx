'use client'

import { JsonEditor as Editor } from 'json-edit-react'
import React, { useEffect } from 'react'
import { Button } from '@/components/ui/button'

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function JsonEditor({ id, secondaryId, attributes, editable, saveAttributes }: { id: string, secondaryId?: string, attributes: any, editable?: boolean, saveAttributes?: ((formData: FormData) => void) | undefined }) {
  const [ attrs, setAttrs ] = React.useState(JSON.stringify(attributes))
  const [ hasDocument, setHasDocument ] = React.useState(false)
  useEffect(() => {
    if (!document) {
      return
    }
    setHasDocument(true)
  }, [hasDocument, setHasDocument])
  return (
    <div className="w-full">
      {hasDocument ?
        <Editor
          data={attributes}
          maxWidth={'100%'}
          className={'w-full'}
          rootName={''}
          enableClipboard={false}
          restrictEdit={saveAttributes === undefined && !editable}
          restrictDelete={saveAttributes === undefined && !editable}
          onUpdate={({ newData }) => {
            setAttrs(JSON.stringify(newData))
          }}
        /> : <></>}
      {saveAttributes ?
        <form action={saveAttributes}>
          <input type="hidden" name="id" value={id}/>
          {secondaryId ? <input type="hidden" name="secondaryId" value={secondaryId}/> : <></>}
          <input type="hidden" name="attributes" value={attrs || '{}'}/>
          <Button size="sm" type="submit" className="mt-4">Save</Button>
        </form> :
        <input type="hidden" name="attributes" value={attrs || '{}'}/>
      }
    </div>
  )
}