import { Metadata } from 'next'
import { CommandMenu } from '@/app/commands/dialog'
import { AppSidebar } from '@/app/components/sidebar/app-sidebar'
import { Selection } from '@/app/components/content/selection'
import { Menu } from '@/app/components/menu'
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from '@/components/ui/resizable'

export const metadata: Metadata = {
  title: 'Bosca',
  description: 'Bosca',
}

// export default async function Page() {
//   return (
//     <>
//       <div className="grid grid-cols-1 h-screen w-screen text-center">
//         {/* eslint-disable-next-line @next/next/no-img-element */}
//         <img src="/bosca.svg" className="w-96 h-96 m-auto" alt="Bosca" />
//         <h1 className="text-4xl">Bosca</h1>
//         <div>Press <Badge variant="outline">Ctrl + Space</Badge> to get started.</div>
//       </div>
//     </>
//   )
// }

export default function Page() {
  return (
    <>
      <ResizablePanelGroup direction="horizontal">
        <ResizablePanel
          defaultSize={10}
          collapsedSize={5}
          collapsible={true}
          minSize={15}
          maxSize={20}
          className="h-screen"
        >
          <AppSidebar />
        </ResizablePanel>
        <ResizableHandle withHandle />
        <ResizablePanel
          defaultSize={90}
          collapsible={false}
          minSize={15}
          maxSize={90}
        >
          <header className="flex h-16 shrink-0 items-center gap-2 px-4 w-full">
            <Menu className={'w-full'} />
          </header>
          <Selection />
        </ResizablePanel>
      </ResizablePanelGroup>
      <CommandMenu actions={[]} current={null}/>
    </>
  )
}
