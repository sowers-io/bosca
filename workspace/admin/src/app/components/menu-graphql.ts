'use server'

import { gql } from '@apollo/client'
import { getClient } from '@/lib/client'

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

export async function addNewCollection(name: string, parentCollectionId: string) {
  const { data } = await getClient().mutate({
    mutation: addCollectionMutation, variables: {
      name: name,
      parentCollectionId: parentCollectionId,
    },
  })
  return data.content.collection.add.id
}

const addMetadataMutation = gql`
    mutation AddMetadata($name: String!, $parentCollectionId: String!) {
        content {
            metadata {
                add(metadata: {
                    parentCollectionId: $parentCollectionId,
                    name: $name,
                    contentType: "metadata",
                    languageTag: "en"
                }) {
                    id
                }
            }
        }
    }
`

export async function addNewMetadata(name: string, parentCollectionId: string) {
  const { data } = await getClient().mutate({
    mutation: addMetadataMutation, variables: {
      name: name,
      parentCollectionId: parentCollectionId,
    },
  })
  return data.content.metadata.add.id
}

const deleteMetadataMutation = gql`
    mutation DeleteMetadata($id: String!) {
        content {
            metadata {
                delete(metadataId: $id)
            }
        }
    }
`

export async function deleteMetadata(id: string) {
  await getClient().mutate({
    mutation: deleteMetadataMutation, variables: {
      id,
    },
  })
}

const deleteCollectionMutation = gql`
    mutation DeleteCollection($id: String!) {
        content {
            collection {
                delete(id: $id, recursive: true)
            }
        }
    }
`

export async function deleteCollection(id: string) {
  await getClient().mutate({
    mutation: deleteCollectionMutation, variables: {
      id,
    },
  })
}