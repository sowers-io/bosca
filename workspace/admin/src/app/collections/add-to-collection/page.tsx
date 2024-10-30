import { Metadata } from 'next'
import { gql } from '@apollo/client'
import { getClient, rootCollectionId } from '@/lib/client'
import { redirect } from 'next/navigation'

export const metadata: Metadata = {
  title: 'Add Collection to Collection',
  description: 'Bosca',
}

const addMutation = gql`
  mutation AddChildCollection($id: String!, $collectionId: String!) {
    content {
      collection {
        addChildCollection(id: $id, collectionId: $collectionId) {
          id
        }
      }
    }
  }
`

export default async function Page({ searchParams } : { searchParams?: { [key: string]: string | string[] | undefined } }) {
  const collection = (searchParams ? searchParams['parent'] as string : null) || rootCollectionId
  const id = (searchParams ? searchParams['id'] as string : null) || rootCollectionId
  const variables = {
    id: collection,
    collectionId: id,
  }
  await getClient().mutate({ mutation: addMutation, variables: variables })
  redirect('/collections/' + id)
}