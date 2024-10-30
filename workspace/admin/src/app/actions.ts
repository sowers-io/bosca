'use server'

import { getClient } from '@/lib/client'
import { gql } from '@apollo/client'

const searchQuery = gql`
    query Search($query: String!, $filter: String, $storageSystemId: String!) {
        content {
            search(query: {query: $query, filter: $filter, offset: 0, limit: 100, storageSystemId: $storageSystemId}) {
                documents {
                    collection {
                        id
                        name
                    }
                    metadata {
                        id
                        name
                    }
                }
            }
        }
    }
`

const storageSystemsQuery = gql`
    query GetStorageSystems {
        workflows {
            storageSystems {
                all {
                    id
                    name
                }
            }
        }
    }
`

export async function onCommandValueChange(value: string, filter: string | null) {
  const client = getClient()
  const storageSystems = await client.query({ query: storageSystemsQuery })
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const storageSystemId = storageSystems.data.workflows.storageSystems.all.filter((s: any) => s.name === 'Default Search')[0].id
  const response = await client.query({ query: searchQuery, variables:{ query: value, filter: filter, storageSystemId: storageSystemId } })
  return response.data.content.search
}
