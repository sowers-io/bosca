import { Metadata } from 'next'
import { redirect } from 'next/navigation'

export const metadata: Metadata = {
  title: 'Metadata',
  description: 'Bosca',
}

export default async function Page() {
  redirect('/')
}