'use client'

import React from 'react'
import { Tag, TagInput } from 'emblor'

export function Labels({ initialLabels }: { initialLabels: string[] | null | undefined }) {
  if (!initialLabels) initialLabels = []
  const [labels, setLabels] = React.useState<Tag[]>(initialLabels.map((l) => { return { 'id': l, 'text': l } }))
  const [activeTagIndex, setActiveTagIndex] = React.useState<number | null>(null)
  return (
    <>
      <input type="hidden" name="labels" value={labels.map((l) => l.text).join(',')} />
      <TagInput
        placeholder="Enter a label"
        tags={labels}
        setTags={(newLabels) => {
          setLabels(newLabels)
        }}
        styleClasses={{
          input: 'w-full border-0 shadow-none',
        }}
        activeTagIndex={activeTagIndex}
        setActiveTagIndex={setActiveTagIndex}
      />
    </>
  )
}