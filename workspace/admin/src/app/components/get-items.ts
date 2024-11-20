'use server'

import { gql } from '@apollo/client'
import { getClient } from '@/lib/client'

const workflowDataQuery = gql`
    query GetWorkflow {
        workflows {
            states {
                all {
                    id
                    name
                }
            }
        }
    }
`

export async function getWorkflowData() {
  const response = (await getClient().query({ query: workflowDataQuery }))
  return response?.data?.workflows
}

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

export async function getItems(id: string) {
  const response = (await getClient().query({ query: collectionQuery, variables: { id: id } }))
  return response?.data?.content?.collection?.items || []
}
