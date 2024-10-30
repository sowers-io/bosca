'use client'

import React from 'react'
import { Button } from '@/components/ui/button'

interface SidebarButtonProps extends React.HTMLAttributes<HTMLDivElement> {
  href: string
}

export default function SidebarButton({ href, children } : SidebarButtonProps) {
  return (
    <Button variant="ghost" className="w-full justify-start" onClick={() => document.location = href}>
      {children}
    </Button>
  )
}