import { Metadata } from 'next'
import { gql } from '@apollo/client'
import { getClient, rootCollectionId } from '@/lib/client'
import { redirect } from 'next/navigation'

export const metadata: Metadata = {
  title: 'Add Metadata',
  description: 'Bosca',
}

const addMetadataMutation = gql`
  mutation AddMetadata($name: String!, $contentType: String!, $languageTag: String!, $parentCollectionId: String!) {
    content {
      metadata {
        add(metadata: {
          parentCollectionId: $parentCollectionId,
          name: $name,
          contentType: $contentType,
          languageTag: $languageTag,
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
    parentCollectionId: parent,
    name: 'New Metadata',
    languageTag: 'en',
    contentType: 'metadata',
  }
  const { data } = await getClient().mutate({ mutation: addMetadataMutation, variables: variables })
  redirect('/metadata/' + data.content.metadata.add.id)
}