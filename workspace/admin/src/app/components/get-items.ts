'use server'

import { gql } from '@apollo/client'
import { getClient } from '@/lib/client'

const collectionQuery = gql`
    query GetCollection($id: String) {
        content {
            collection(id: $id) {
                id
                name
                labels
                created
                modified
                attributes
                ready
                public
                publicList
                permissions {
                    action
                    group {
                        id
                        name
                    }
                }
                ordering
                items(offset: 0, limit: 1000) {
                    __typename
                    ... on Collection {
                        id
                        name
                        labels
                        itemAttributes
                    }
                    ... on Metadata {
                        id
                        name
                        labels
                        itemAttributes
                    }
                }
            }
        }
    }
`

export async function getItems(id: string) {
  'use server'
  const response = (await getClient().query({ query: collectionQuery, variables: { id: id } }))
  return response?.data?.content?.collection?.items || []
}
