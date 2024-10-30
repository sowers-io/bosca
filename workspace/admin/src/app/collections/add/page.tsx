import { Metadata } from 'next'
import { gql } from '@apollo/client'
import { getClient, rootCollectionId } from '@/lib/client'
import { redirect } from 'next/navigation'

export const metadata: Metadata = {
  title: 'Collections',
  description: 'Bosca',
}

const addCollectionMutation = gql`
  mutation AddCollection($name: String!, $parentCollectionId: String!) {
    content {
      collection {
        add(collection: {
          parentCollectionId: $parentCollectionId,
          name: $name
        }) {
          id
        }
      }
    }
  }
`

export default async function Page({ searchParams } : { searchParams?: { [key: string]: string | string[] | undefined } }) {
  const parent = (searchParams ? searchParams['parent'] as string : null) || rootCollectionId
  const variables = {
    parentCollectionId: parent || rootCollectionId,
    name: 'New Collection',
  }
  const { data } = await getClient().mutate({ mutation: addCollectionMutation, variables: variables })
  redirect('/collections/' + data.content.collection.add.id)
}