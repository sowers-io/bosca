import { Metadata } from 'next'
import { redirect } from 'next/navigation'

export const metadata: Metadata = {
  title: 'Collections',
  description: 'Bosca',
}

export default async function Page() {
  redirect('/')
}