'use client'

import { buttonVariants } from '@/components/ui/button'
import React from 'react'
import { Slot } from '@radix-ui/react-slot'
import { cn } from '@/lib/utils'
import type { VariantProps } from 'class-variance-authority'

export interface ButtonLinkProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
  VariantProps<typeof buttonVariants> {
  href: string
  asChild?: boolean
}


const ButtonLink = React.forwardRef<HTMLButtonElement, ButtonLinkProps>(
  ({ href, className, variant, size, asChild = false, ...props }, ref) => {
    const Comp = asChild ? Slot : 'button'
    return (
      <Comp
        className={cn(buttonVariants({ variant, size, className }))}
        onClick={() => document.location = href}
        type="button"
        ref={ref}
        {...props}
      />
    )
  }
)
ButtonLink.displayName = 'Button'

export { ButtonLink }