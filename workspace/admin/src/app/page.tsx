import { Metadata } from 'next'
import { CommandMenu } from '@/app/commands/dialog'
import { Badge } from '@/components/ui/badge'

export const metadata: Metadata = {
  title: 'Bosca',
  description: 'Bosca',
}

export default async function Page() {
  return ( 
    <>
      <div className="grid grid-cols-1 h-screen w-screen text-center">
        {/* eslint-disable-next-line @next/next/no-img-element */}
        <img src="/bosca.svg" className="w-96 h-96 m-auto" alt="Bosca" />
        <h1 className="text-4xl">Bosca</h1>
        <div>Press <Badge variant="outline">Ctrl + Space</Badge> to get started.</div>
      </div>
      <CommandMenu actions={[]} current={null}  />
    </>
  )
}