'use client'

import React, { useState } from 'react'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from '@/components/ui/card'
import { Label } from '@/components/ui/label'

interface LoginFormProps extends React.HTMLAttributes<HTMLDivElement> {
  login: (formData: FormData) => Promise<void>
}

export function LoginForm({ login }: LoginFormProps) {
  'use client'
  const [submitting, setSubmitting] = useState(false)
  return (
    <form action={login} className="space-y-8 w-96" onSubmit={() => setSubmitting(true)}>
      <Card className="w-full max-w-sm">
        <CardHeader>
          <CardTitle className="text-2xl">Login</CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4">
          <div className="grid gap-2">
            <Label htmlFor="username">Username</Label>
            <Input id="username" name="username" type="text" required />
          </div>
          <div className="grid gap-2">
            <Label htmlFor="password">Password</Label>
            <Input id="password" name="password" type="password" required />
          </div>
        </CardContent>
        <CardFooter>
          <Button type="submit" disabled={submitting} className="w-full">Login</Button>
        </CardFooter>
      </Card>
    </form>
  )
}