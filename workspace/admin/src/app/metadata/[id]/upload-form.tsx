'use client'

import React, { useEffect, useState } from 'react'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'

interface UploadFormProps extends React.HTMLAttributes<HTMLDivElement> {
  id: string
}

export function UploadForm({ id }: UploadFormProps) {
  'use client'
  const [submitting, setSubmitting] = useState(false)
  const [currentUrl, setCurrentUrl] = useState<string | null>(null)
  const [uploadUrl, setUploadUrl] = useState<string | null>(null)
  useEffect(() => {
    if (window.location.host === '127.0.0.1:3000' || window.location.host === 'localhost:3000') {
      setUploadUrl('http://127.0.0.1:8000')
    } else {
      setUploadUrl('https://' + window.location.host.replaceAll('admin.', 'api.'))
    }
    setCurrentUrl(encodeURIComponent(window.location.href))
  }, [])
  return (
    <form action={uploadUrl + '/files/upload?id=' + id + '&redirect=' + currentUrl} encType={'multipart/form-data'} method="POST" className="space-y-8" onSubmit={() => setSubmitting(true)}>
      <Input placeholder="Name" name="name" type="file"/>
      <Button type="submit" disabled={submitting}>Upload</Button>
    </form>
  )
}