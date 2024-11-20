/* eslint-disable @typescript-eslint/no-explicit-any */
import * as React from 'react'

import Tree from '@/app/components/sidebar/app-sidebar-tree'
import { gql } from '@apollo/client'
import { getClient } from '@/lib/client'
import SearchForm from '@/app/components/sidebar/app-sidebar-search'

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
            parentCollections(offset: 0, limit: 100) {
              id
              name
            }
            workflow {
              state
              pending
              plans {
                id
                complete {
                  index
                  queue
                }
                pending {
                  index
                  queue
                }
                running {
                  index
                  queue
                }
                workflow {
                  id
                  name
                }
              }
            }
          }
          ... on Metadata {
            id
            version
            name
            labels
            languageTag
            parentCollections(offset: 0, limit: 100) {
              id
              name
            }
            content {
              type
              urls {
                download {
                  url
                  headers {
                    name
                    value
                  }
                }
              }
            }
            public
            publicContent
            publicSupplementary
            permissions {
              action
              group {
                id
                name
              }
            }
            created
            modified
            uploaded
            ready
            attributes
            systemAttributes
            relationships {
              metadata {
                id
                name
              }
              relationship
              attributes
            }
            supplementary {
              key
              name
              uploaded
              content {
                type
                urls {
                  download {
                    url
                    headers {
                      name
                      value
                    }
                  }
                }
              }
            }
            traitIds
            workflow {
              state
              pending
              plans {
                id
                complete {
                  index
                  queue
                }
                pending {
                  index
                  queue
                }
                running {
                  index
                  queue
                }
                workflow {
                  id
                  name
                }
              }
            }
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

