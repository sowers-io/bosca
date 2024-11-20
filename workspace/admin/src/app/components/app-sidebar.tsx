/* eslint-disable @typescript-eslint/no-explicit-any */
import * as React from 'react'

import Tree from '@/app/components/app-sidebar-tree'
import { gql } from '@apollo/client'
import { getClient } from '@/lib/client'
import SearchForm from '@/app/components/app-sidebar-search'

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

export async function AppSidebar() {
  const response = (await getClient().query({ query: collectionQuery }))
  const data = response.data
  return (
    <div className="p-2">
      <SearchForm />
      <Tree item={data.content.collection} />
    </div>
  )
}

