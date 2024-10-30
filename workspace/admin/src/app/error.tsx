'use client'

import { redirect } from 'next/navigation'
import { cn } from '@/lib/utils'
import { fontSans } from '@/lib/fonts'
import { ThemeProvider } from 'next-themes'
import { ErrorDialog } from '@/app/error-dialog'
import React from 'react'

export default function Error({ error }: { error: Error & { digest?: string } }) {
  if (error.message === 'invalid permissions') {
    redirect('/user/login')
  }
  return (
    <>
      <html lang="en" suppressHydrationWarning>
        <head />
        <body
          className={cn(
            'min-h-screen bg-background font-sans antialiased',
            fontSans.variable
          )}
        >
          <ThemeProvider
            attribute="class"
            defaultTheme="system"
            enableSystem
            disableTransitionOnChange
          >
            <ErrorDialog error={error.message} location={'/'} />
          </ThemeProvider>
        </body>
      </html>
    </>
  )
}