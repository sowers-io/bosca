import { Metadata } from 'next'
import { getClient, rootCollectionId } from '@/lib/client'
import { gql } from '@apollo/client'
import { ButtonLink } from '@/components/button-link'
import { ChevronLeft } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Label } from '@/components/ui/label'
import { Input } from '@/components/ui/input'
import { Textarea } from '@/components/ui/textarea'
import { Button } from '@/components/ui/button'
import { redirect } from 'next/navigation'
import { CommandMenu } from '@/app/commands/dialog'
import { ErrorDialog } from '@/app/error-dialog'
import { States } from '@/app/collections/[id]/states'
import { JsonEditor } from '@/components/json-editor'
import React from 'react'
import { Labels } from '@/components/labels'

export const metadata: Metadata = {
  title: 'Collections',
  description: 'Bosca',
}

const collectionQuery = gql`
    query GetCollection($id: String!) {
        content {
            traits {
                id
                name
            }
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
                parentCollections(offset: 0, limit: 100) {
                    id
                    name
                }
                collections(offset: 0, limit: 1000) {
                    id
                    name
                    description
                    type
                    labels
                    itemAttributes
                }
                metadata(offset: 0, limit: 1000) {
                    id
                    name
                    type
                    labels
                    itemAttributes
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
        }
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

const collectionMutation = gql`
    mutation EditCollection($id: String!, $name: String!, $labels: [String!], $description: String!) {
        content {
            collection {
                edit(
                    id: $id,
                    collection: {
                        name: $name,
                        description: $description
                        labels: $labels,
                    }) {
                    id
                }
            }
        }
    }
`

const setPublicMutation = gql`
    mutation SetCollectionPublic($id: String!, $public: Boolean!) {
        content {
            collection {
                setPublic(id: $id, public: $public) {
                    id
                }
            }
        }
    }
`

const setPublicListMutation = gql`
    mutation SetCollectionPublic($id: String!, $public: Boolean!) {
        content {
            collection {
                setPublicList(id: $id, public: $public) {
                    id
                }
            }
        }
    }
`

const deleteMutation = gql`
    mutation DeleteCollection($id: String!) {
        content {
            collection {
                delete(id: $id, recursive: true)
            }
        }
    }
`

const removeMetadataMutation = gql`
    mutation RemoveMetadata($collectionId: String!, $id: String!) {
        content {
            collection {
                removeChildMetadata(id: $collectionId, metadataId: $id) {
                    id
                }
            }
        }
    }
`

const setStateMutation = gql`
    mutation SetState($id: String!, $state: String!) {
        workflows {
            beginTransition(request: {collectionId: $id, stateId: $state, status: "User Update", waitForCompletion: true})
        }
    }
`

const markReadyMutation = gql`
    mutation SetMetadataReady($id: String!) {
        content {
            collection {
                setReady(id: $id)
            }
        }
    }
`

const removeCollectionMutation = gql`
    mutation RemoveCollection($collectionId: String!, $id: String!) {
        content {
            collection {
                removeChildCollection(id: $collectionId, collectionId: $id) {
                    id
                }
            }
        }
    }
`

const setOrderingMutation = gql`
    mutation SetOrdering($id: String!, $ordering: JSON!) {
        content {
            collection {
                setCollectionOrdering(id: $id, ordering: $ordering)
            }
        }
    }
`

const setAttributesMutation = gql`
    mutation SetAttributes($id: String!, $attributes: JSON!) {
        content {
            collection {
                setCollectionAttributes(id: $id, attributes: $attributes)
            }
        }
    }
`

const setItemAttributesMutation = gql`
    mutation SetItemAttributes($id: String!, $collectionId: String, $metadataId: String, $attributes: JSON!) {
        content {
            collection {
                setChildItemAttributes(id: $id, childCollectionId: $collectionId, childMetadataId: $metadataId, attributes: $attributes) {
                    id
                }
            }
        }
    }
`

export default async function Page({ params, searchParams }: {
  params: { id: string },
  searchParams?: { [key: string]: string | string[] | undefined }
}) {
  const variables = { id: params.id }

  async function save(formData: FormData) {
    'use server'
    const labels = !formData.get('labels') || formData.get('labels')!.toString().trim().length == 0 ? [] : formData.get('labels')!.toString().split(',').map((l) => l.trim())
    const variables = {
      id: formData.get('id'),
      name: formData.get('name'),
      labels: labels,
      description: formData.get('description'),
    }
    const { data } = await getClient().mutate({ mutation: collectionMutation, variables: variables })
    redirect('/collections/' + data.content.collection.edit.id)
  }

  async function setOrdering(formData: FormData) {
    'use server'
    const ordering = JSON.parse(formData.get('attributes')?.toString() || '[]')
    const variables = {
      id: formData.get('id'),
      ordering: ordering,
    }
    await getClient().mutate({ mutation: setOrderingMutation, variables: variables })
    redirect('/collections/' + formData.get('id'))
  }

  async function setAttributes(formData: FormData) {
    'use server'
    const attributes = JSON.parse(formData.get('attributes')!.toString())
    const variables = {
      id: formData.get('id'),
      attributes: attributes,
    }
    await getClient().mutate({ mutation: setAttributesMutation, variables: variables })
    redirect('/collections/' + formData.get('id'))
  }

  async function setCollectionItemAttributes(formData: FormData) {
    'use server'
    const attributes = JSON.parse(formData.get('attributes')!.toString())
    const variables = {
      id: formData.get('id'),
      collectionId: formData.get('secondaryId'),
      attributes: attributes,
    }
    await getClient().mutate({ mutation: setItemAttributesMutation, variables: variables })
    redirect('/collections/' + formData.get('id'))
  }

  async function setMetadataItemAttributes(formData: FormData) {
    'use server'
    const attributes = JSON.parse(formData.get('attributes')!.toString())
    const variables = {
      id: formData.get('id'),
      metadataId: formData.get('secondaryId'),
      attributes: attributes,
    }
    await getClient().mutate({ mutation: setItemAttributesMutation, variables: variables })
    redirect('/collections/' + formData.get('id'))
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  let error: any | null = null
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  let data: any
  try {
    if (searchParams && searchParams['delete'] === 'true') {
      const { data } = await getClient().query({ query: collectionQuery, variables: { id: params.id } })
      await getClient().mutate({ mutation: deleteMutation, variables: { id: params.id } })
      redirect('/collections/' + (data.content.collection.parentCollections[0]?.id || rootCollectionId))
    }
    if (searchParams && searchParams['set-public']) {
      await getClient().mutate({
        mutation: setPublicMutation,
        variables: { id: params.id, public: searchParams['set-public'] === 'true' },
      })
      redirect('/collections/' + params.id)
    }
    if (searchParams && searchParams['set-public-list']) {
      await getClient().mutate({
        mutation: setPublicListMutation,
        variables: { id: params.id, public: searchParams['set-public-list'] === 'true' },
      })
      redirect('/collections/' + params.id)
    }
    if (searchParams && searchParams['set-state']) {
      await getClient().mutate({
        mutation: setStateMutation,
        variables: { id: params.id, state: searchParams['set-state'] },
      })
      redirect('/collections/' + params.id)
    }
    if (searchParams && searchParams['remove-collection']) {
      await getClient().mutate({
        mutation: removeCollectionMutation,
        variables: { collectionId: searchParams['remove-collection'], id: params.id },
      })
      redirect('/collections/' + params.id)
    }
    if (searchParams && searchParams['remove-metadata']) {
      await getClient().mutate({
        mutation: removeMetadataMutation,
        variables: { collectionId: searchParams['remove-metadata'], id: params.id },
      })
      redirect('/collections/' + params.id)
    }
    if (searchParams && searchParams['ready'] === 'true') {
      await getClient().mutate({ mutation: markReadyMutation, variables: { id: params.id } })
      redirect('/collections/' + params.id)
    }
    const response = (await getClient().query({ query: collectionQuery, variables: variables }))
    data = response.data
    if (!(data?.content?.collection)) {
      if (response.error) {
        throw new Error(response.error.toString())
      }
      if (response.errors && response.errors.length > 0) {
        throw new Error(response.errors.map((e) => e.message).join(', '))
      }
      throw new Error('collection not found')
    }
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (e: any) {
    if (e.message === 'NEXT_REDIRECT') {
      throw e
    }
    if (e.message === 'invalid permissions') {
      redirect('/user/login')
    }
    if (e.message === 'collection not found') {
      redirect('/?error=' + encodeURIComponent(e.message))
    }
    error = e
  }
  return (
    <>
      {error ? <ErrorDialog error={error.message} location={error.message.indexOf('collection not found:') != -1 ? '/' : '/collections/' + params.id} /> :
        <div className="border-t">
          <div className="bg-background">
            <div className="col-span-3 lg:col-span-4 lg:border-l">
              <div className="h-full px-4 py-6 lg:px-8">
                <div className="flex items-center justify-between">
                  <div className="mx-auto grid flex-1 gap-4">
                    <div className="flex items-center gap-4">
                      <ButtonLink
                        href={'/collections/' + (data.content.collection.parentCollections[0]?.id || '')}
                        variant="outline" size="icon"
                        className="h-7 w-7">
                        <ChevronLeft className="h-4 w-4"/>
                        <span className="sr-only">Back</span>
                      </ButtonLink>
                      <h1 className="flex-1 shrink-0 whitespace-nowrap text-xl font-semibold tracking-tight sm:grow-0">
                        {data.content.collection.name}
                      </h1>
                      <Badge variant="outline" className="ml-auto sm:ml-0">
                        {data.content.collection.workflow.state}
                      </Badge>
                    </div>
                    <div className="grid grid-cols-2 gap-4">
                      <div className="grid gap-4">
                        <Card x-chunk="dashboard-07-chunk-0">
                          <CardHeader>
                            <CardTitle>Collection Details</CardTitle>
                            <CardDescription>
                              {data.content.collection.description || ''}
                            </CardDescription>
                          </CardHeader>
                          <CardContent>
                            <form action={save}
                              className="grid auto-rows-max items-start gap-4 grow">
                              <input type="hidden" name="id"
                                value={data.content.collection.id}/>
                              <div className="grid gap-6">
                                <div className="grid gap-3">
                                  <Label htmlFor="name">Name</Label>
                                  <Input
                                    id="name"
                                    name="name"
                                    type="text"
                                    className="w-full"
                                    defaultValue={data.content.collection.name}
                                  />
                                </div>
                                <div className="grid gap-3">
                                  <Label htmlFor="labels">Labels</Label>
                                  <Labels
                                    initialLabels={data.content.collection.labels}/>
                                </div>
                                <div className="grid gap-3">
                                  <Label htmlFor="name">Created</Label>
                                  {data.content.collection.created}
                                </div>
                                <div className="grid gap-3">
                                  <Label htmlFor="name">Modified</Label>
                                  {data.content.collection.modified}
                                </div>
                                <div className="grid gap-3">
                                  <Label htmlFor="description">Description</Label>
                                  <Textarea
                                    id="description"
                                    name="description"
                                    defaultValue={data.content.collection.description || ''}
                                    className="min-h-32"
                                  />
                                </div>
                                <div className="grid grid-cols-2 gap-6">
                                  <ButtonLink
                                    href={'/collections/' + data.content.collection.id + '?delete=true'}
                                    variant="outline" size="sm">
                                    Delete
                                  </ButtonLink>
                                  <Button size="sm" type="submit">Save
                                    Collection</Button>
                                </div>
                              </div>
                            </form>
                          </CardContent>
                        </Card>
                        <Card x-chunk="dashboard-07-chunk-0">
                          <CardHeader>
                            <CardTitle>Collection Ordering</CardTitle>
                          </CardHeader>
                          <CardContent>
                            <JsonEditor
                              id={data.content.collection.id}
                              attributes={data.content.collection.ordering}
                              saveAttributes={setOrdering}/>
                          </CardContent>
                        </Card>
                        <Card x-chunk="dashboard-07-chunk-0">
                          <CardHeader>
                            <CardTitle>Collection Attributes</CardTitle>
                          </CardHeader>
                          <CardContent>
                            <JsonEditor
                              id={data.content.collection.id}
                              attributes={data.content.collection.attributes}
                              saveAttributes={setAttributes}/>
                          </CardContent>
                        </Card>
                      </div>
                      <div className="grid auto-rows-max items-start gap-4">
                        <Card x-chunk="dashboard-07-chunk-3">
                          <CardHeader>
                            <CardTitle>Collection State</CardTitle>
                          </CardHeader>
                          <CardContent>
                            <div className="grid gap-6">
                              <div className="grid gap-3">
                                <States id={data.content.collection.id}
                                  states={data.workflows.states.all}
                                  current={data.content.collection.workflow.state}
                                  pending={data.content.collection.workflow.pending}
                                />
                              </div>
                              {
                                data.content.collection.ready ?
                                  <></> :
                                  <ButtonLink
                                    href={'/collections/' + data.content.collection.id + '?ready=true'}>Mark
                                    Ready</ButtonLink>
                              }
                            </div>
                          </CardContent>
                        </Card>
                        <Card x-chunk="dashboard-07-chunk-3">
                          <CardHeader>
                            <CardTitle>Collection Permissions</CardTitle>
                          </CardHeader>
                          <CardContent>
                            Is Public?: <a
                              href={'/collections/' + data.content.collection.id + '?set-public=' + (!data.content.collection['public'])}>{data.content.collection['public'].toString()}</a><br/>
                            Is Public List?: <a
                              href={'/collections/' + data.content.collection.id + '?set-public-list=' + (!data.content.collection['publicList'])}>{data.content.collection['publicList'].toString()}</a>
                            <table className="mt-2">
                              <tbody>
                                {
                                  // eslint-disable-next-line @typescript-eslint/no-explicit-any
                                  data.content.collection.permissions.map((p: any) => {
                                    return (
                                      <tr key={p.action + '-' + p.group.id}>
                                        <td className="w-64">{p.action}</td>
                                        <td className="w-64">{p.group.name}</td>
                                      </tr>
                                    )
                                  })
                                }
                              </tbody>
                            </table>
                          </CardContent>
                        </Card>
                        <Card x-chunk="dashboard-07-chunk-0">
                          <CardHeader>
                            <CardTitle>Collections</CardTitle>
                          </CardHeader>
                          <CardContent>
                            <table className="w-full">
                              <tbody>
                                {
                                  // eslint-disable-next-line @typescript-eslint/no-explicit-any
                                  data.content.collection.collections.map((collection: any) =>
                                    <>
                                      <tr key={collection.id}>
                                        <td className="p-2"><a
                                          href={'/collections/' + collection.id}>{collection.name}</a>
                                        </td>
                                      </tr>
                                      <tr key={collection.id + '-attributes'}>
                                        <td>
                                          <JsonEditor
                                            id={data.content.collection.id}
                                            secondaryId={collection.id}
                                            attributes={collection.itemAttributes}
                                            saveAttributes={setCollectionItemAttributes}/>
                                        </td>
                                      </tr>
                                    </>
                                  )
                                }
                              </tbody>
                            </table>
                          </CardContent>
                        </Card>
                        <Card x-chunk="dashboard-07-chunk-0">
                          <CardHeader>
                            <CardTitle>Metadata</CardTitle>
                          </CardHeader>
                          <CardContent>
                            <table className="w-full">
                              <tbody>
                                {
                                  // eslint-disable-next-line @typescript-eslint/no-explicit-any
                                  data.content.collection.metadata.map((metadata: any) =>
                                    <>
                                      <tr key={metadata.id}>
                                        <td className="p-2"><a
                                          href={'/metadata/' + metadata.id}>{metadata.name}</a>
                                        </td>
                                      </tr>
                                      <tr key={metadata.id + '-attributes'}>
                                        <td>
                                          <JsonEditor
                                            id={data.content.collection.id}
                                            secondaryId={metadata.id}
                                            attributes={metadata.itemAttributes}
                                            saveAttributes={setMetadataItemAttributes}/>
                                        </td>
                                      </tr>
                                    </>
                                  )
                                }
                              </tbody>
                            </table>
                          </CardContent>
                        </Card>
                        <Card x-chunk="dashboard-07-chunk-0">
                          <CardHeader>
                            <CardTitle>Parent Collections</CardTitle>
                          </CardHeader>
                          <CardContent>
                            <table>
                              <tbody>
                                {
                                  // eslint-disable-next-line @typescript-eslint/no-explicit-any
                                  data.content.collection.parentCollections.map((collection: any) =>
                                    <tr key={collection.id}>
                                      <td className="p-2"><a
                                        href={'/collections/' + collection.id}>{collection.name}</a>
                                      </td>
                                      <td>
                                        <ButtonLink
                                          href={'/collections/' + data.content.collection.id + '?remove-collection=' + collection.id}
                                          variant="outline"
                                          size="sm">Remove</ButtonLink>
                                      </td>
                                    </tr>
                                  )
                                }
                              </tbody>
                            </table>
                          </CardContent>
                        </Card>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      }
      <CommandMenu actions={[]} current={(data && data.content && data.content.collection) ? {
        id: params.id,
        item: data.content.collection,
        collection: true,
      } : null}/>
    </>
  )
}
