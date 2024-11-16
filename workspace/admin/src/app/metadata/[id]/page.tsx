import { Metadata } from 'next'
import { getClient, rootCollectionId } from '@/lib/client'
import { gql } from '@apollo/client'

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Button } from '@/components/ui/button'
import { ChevronLeft  } from 'lucide-react'
import { Badge } from '@/components/ui/badge'
import { ButtonLink } from '@/components/button-link'
import { redirect } from 'next/navigation'
import { UploadForm } from '@/app/metadata/[id]/upload-form'
import { Traits } from '@/app/metadata/[id]/traits'
import { States } from '@/app/metadata/[id]/states'
import { CommandMenu } from '@/app/commands/dialog'
import { ErrorDialog } from '@/app/error-dialog'
import { JsonEditor } from '@/components/json-editor'
import { Labels } from '@/components/labels'
import React from 'react'
import { Relationship } from '@/app/metadata/[id]/relationship'
import { Workflows } from '@/app/metadata/[id]/workflows'

export const metadata: Metadata = {
  title: 'Metadata',
  description: 'Bosca',
}

const metadataQuery = gql`
  query GetMetadata($id: String!) {
    content {
      traits {
        id
        name
      }
      metadata(id: $id) {
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
    workflows {  
      all {
        id
        name
      }
      states {
        all {
          id
          name
        }
      }
    }
  }
`

const metadataMutation = gql`
  mutation EditMetadata($id: String!, $name: String!, $labels: [String!], $contentType: String!, $languageTag: String!, $attributes: JSON!) {
    content {
      metadata {
        edit(
          id: $id,
          metadata: {
            name: $name,
            contentType: $contentType,
            languageTag: $languageTag,
            attributes: $attributes,
            labels: $labels
          }) {
          id
        }
      }
    }
  }
`

const markReadyMutation = gql`
  mutation SetMetadataReady($id: String!) {
    content {
      metadata {
        setMetadataReady(id: $id)
      }
    }
  }
`

const executeWorkflow = gql`
  mutation ExecuteWorkflow($id: String!, $version: Int!, $workflowId: String!) {
    workflows {
      enqueueWorkflow(metadataId: $id, version: $version, workflowId: $workflowId) {
        id
      }
    }
  }
`

const setPublicMutation = gql`
  mutation SetMetadataPublic($id: String!, $public: Boolean!) {
    content {
      metadata {
        setPublic(id: $id, public: $public) {
          id
        }
      }
    }
  }
`

const setPublicContentMutation = gql`
  mutation SetMetadataPublic($id: String!, $public: Boolean!) {
    content {
      metadata {
        setPublicContent(id: $id, public: $public) {
          id
        }
      }
    }
  }
`

const setPublicSupplementaryMutation = gql`
  mutation SetMetadataPublic($id: String!, $public: Boolean!) {
    content {
      metadata {
        setPublicSupplementary(id: $id, public: $public) {
          id
        }
      }
    }
  }
`

const setStateMutation = gql`
  mutation SetState($id: String!, $version: Int!, $state: String!) {
    workflows {
      beginTransition(request: {metadataId: $id, version: $version, stateId: $state, status: "User Update", waitForCompletion: true})
    }
  }
`

const addTraitMutation = gql`
  mutation AddTrait($id: String!, $traitId: String!) {
    content {
      metadata {
        addTrait(metadataId: $id, traitId: $traitId) {
          metadataId
        }
      }
    }
  }
`

const deleteTraitMutation = gql`
  mutation DeleteTrait($id: String!, $traitId: String!) {
    content {
      metadata {
        deleteTrait(metadataId: $id, traitId: $traitId) {
          metadataId
        }
      }
    }
  }
`

const deleteMutation = gql`
  mutation DeleteMetadata($id: String!) {
    content {
      metadata {
        delete(metadataId: $id)
      }
    }
  }
`

const deleteContentMutation = gql`
  mutation DeleteContentMetadata($id: String!) {
    content {
      metadata {
        deleteContent(metadataId: $id)
      }
    }
  }
`

const removeCollectionMutation = gql`
  mutation RemoveCollection($collectionId: String!, $id: String!) {
    content {
      collection {
        removeChildMetadata(id: $collectionId, metadataId: $id) {
          id
        }
      }
    }
  }
`

const setAttributesMutation = gql`
  mutation SetAttributes($id: String!, $attributes: JSON!) {
    content {
      metadata {
         setMetadataAttributes(id: $id, attributes: $attributes)
      }
    }
  }
`

const addRelationshipMutation = gql`
  mutation AddRelationship($relationship: MetadataRelationshipInput!) {
    content {
      metadata {
        addRelationship(relationship: $relationship) {
          id
        }
      }
    }
  }
`

const deleteRelationshipMutation = gql`
  mutation DeleteRelationship($id1: String!, $id2: String!, $relationship: String!) {
    content {
      metadata {
        deleteRelationship(id1: $id1, id2: $id2, relationship: $relationship)
      }
    }
  }
`

export default async function Page({ params, searchParams }: { params: { id: string }, searchParams?: { [key: string]: string | string[] | undefined } }) {
  async function save(formData: FormData) {
    'use server'
    const { data: d } = await getClient().query({ query: metadataQuery, variables: { id: formData.get('id') } })
    let attributes = d.content.metadata.attributes
    if (!attributes) {
      attributes = {}
    }
    const labels = !formData.get('labels') || formData.get('labels')!.toString().trim().length == 0 ? [] : formData.get('labels')!.toString().split(',').map((l) => l.trim())
    const variables = {
      id: formData.get('id'),
      name: formData.get('name'),
      labels: labels,
      languageTag: formData.get('languageTag'),
      contentType: formData.get('contentType'),
      attributes: attributes,
    }
    const { data } = await getClient().mutate({ mutation: metadataMutation, variables: variables })
    redirect('/metadata/' + data.content.metadata.edit.id)
  }
  async function setAttributes(formData: FormData) {
    'use server'
    let attributes = JSON.parse(formData.get('attributes')!.toString())
    if (!attributes) {
      attributes = {}
    }
    const variables = {
      id: formData.get('id'),
      attributes: attributes,
    }
    await getClient().mutate({ mutation: setAttributesMutation, variables: variables })
    redirect('/metadata/' + formData.get('id'))
  }
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  let error: any | null = null
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  let data: any
  try {
    if (searchParams && searchParams['delete'] === 'true') {
      const { data } = await getClient().query({ query: metadataQuery, variables: { id: params.id } })
      await getClient().mutate({ mutation: deleteMutation, variables: { id: params.id } })
      redirect('/collections/' + (data.content.metadata.parentCollections[0]?.id || rootCollectionId))
    }
    if (searchParams && searchParams['add-trait']) {
      await getClient().mutate({
        mutation: addTraitMutation,
        variables: { id: params.id, traitId: searchParams['add-trait'] },
      })
      redirect('/metadata/' + params.id)
    }
    if (searchParams && searchParams['add-relationship']) {
      await getClient().mutate({
        mutation: addRelationshipMutation,
        variables: {
          relationship: {
            id1: params.id,
            id2: searchParams['add-relationship'],
            relationship: '',
            attributes: {},
          },
        },
      })
      await getClient().mutate({
        mutation: addRelationshipMutation,
        variables: {
          relationship: {
            id1: searchParams['add-relationship'],
            id2: params.id,
            relationship: '',
            attributes: {},
          },
        },
      })
      redirect('/metadata/' + params.id)
    }
    if (searchParams && searchParams['delete-relationship']) {
      await getClient().mutate({
        mutation: deleteRelationshipMutation,
        variables: { id1: params.id, id2: searchParams['delete-relationship'], relationship: searchParams['relationship'] },
      })
      redirect('/metadata/' + params.id)
    }
    if (searchParams && searchParams['set-public']) {
      await getClient().mutate({
        mutation: setPublicMutation,
        variables: { id: params.id, public: searchParams['set-public'] === 'true' },
      })
      redirect('/metadata/' + params.id)
    }
    if (searchParams && searchParams['set-public-content']) {
      await getClient().mutate({
        mutation: setPublicContentMutation,
        variables: { id: params.id, public: searchParams['set-public-content'] === 'true' },
      })
      redirect('/metadata/' + params.id)
    }
    if (searchParams && searchParams['set-public-supplementary']) {
      await getClient().mutate({
        mutation: setPublicSupplementaryMutation,
        variables: { id: params.id, public: searchParams['set-public-supplementary'] === 'true' },
      })
      redirect('/metadata/' + params.id)
    }
    if (searchParams && searchParams['set-state']) {
      await getClient().mutate({
        mutation: setStateMutation,
        variables: { id: params.id, state: searchParams['set-state'], version: parseInt(searchParams['version']!.toString()) },
      })
      redirect('/metadata/' + params.id)
    }
    if (searchParams && searchParams['delete-trait']) {
      await getClient().mutate({
        mutation: deleteTraitMutation,
        variables: { id: params.id, traitId: searchParams['delete-trait'] },
      })
      redirect('/metadata/' + params.id)
    }
    if (searchParams && searchParams['delete-content'] === 'true') {
      await getClient().mutate({ mutation: deleteContentMutation, variables: { id: params.id } })
      redirect('/metadata/' + params.id)
    }
    if (searchParams && searchParams['remove-collection']) {
      await getClient().mutate({ mutation: removeCollectionMutation, variables: { collectionId: searchParams['remove-collection'], id: params.id } })
      redirect('/metadata/' + params.id)
    }
    if (searchParams && searchParams['ready'] === 'true') {
      await getClient().mutate({ mutation: markReadyMutation, variables: { id: params.id } })
      redirect('/metadata/' + params.id)
    }
    if (searchParams && searchParams['workflow']) {
      const { data } = await getClient().query({ query: metadataQuery, variables: { id: params.id } })
      await getClient().mutate({ mutation: executeWorkflow, variables: { id: params.id, version: data.content.metadata.version, workflowId: searchParams['workflow'] } })
      redirect('/metadata/' + params.id)
    }
    const response = await getClient().query({ query: metadataQuery, variables: { id: params.id } })
    data = response.data
    if (!(data?.content?.metadata)) {
      if (response.error) {
        throw new Error(response.error.toString())
      }
      if (response.errors && response.errors.length > 0) {
        throw new Error(response.errors.map((e) => e.message).join(', '))
      }
      throw new Error('metadata not found')
    }
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  } catch (e: any) {
    if (e.message === 'NEXT_REDIRECT') {
      throw e
    }
    if (e.message === 'invalid permissions') {
      redirect('/user/login')
    }
    if (e.message === 'metadata not found') {
      redirect('/?error=' + encodeURIComponent(e.message))
    }
    error = e
  }
  return (
    <>
      {error ? <ErrorDialog error={error.message} location={error.message.indexOf('metadata not found:') != -1 ? '/' : '/metadata/' + params.id} /> :
        <div className="border-t">
          <div className="bg-background">
            <div className="col-span-3 lg:col-span-4 lg:border-l">
              <div className="h-full px-4 py-6 lg:px-8">
                <div className="flex items-center justify-between">
                  <div className="mx-auto grid flex-1 gap-4">
                    <div className="flex items-center gap-4">
                      <ButtonLink href={'/collections/' + (data.content.metadata.parentCollections[0]?.id || rootCollectionId)} variant="outline" size="icon"
                        className="h-7 w-7">
                        <ChevronLeft className="h-4 w-4"/>
                        <span className="sr-only">Back</span>
                      </ButtonLink>
                      <h1 className="flex-1 shrink-0 whitespace-nowrap text-xl font-semibold tracking-tight sm:grow-0">
                        {data.content.metadata.name}
                      </h1>
                      <Badge variant="outline" className="ml-auto sm:ml-0">
                        {data.content.metadata.workflow.state}
                      </Badge>
                    </div>
                    <div className="grid grid-cols-2 gap-4">
                      <div className="grid auto-rows-max items-start gap-4">
                        <Card x-chunk="dashboard-07-chunk-0">
                          <CardHeader>
                            <CardTitle>Metadata Details</CardTitle>
                            <CardDescription>
                              {data.content.metadata.attributes?.description || ''}
                            </CardDescription>
                          </CardHeader>
                          <CardContent>
                            <form action={save} className="grid auto-rows-max items-start gap-4 grow">
                              <input type="hidden" name="id" value={data.content.metadata.id}/>
                              <div className="grid gap-6">
                                <div className="grid gap-3">
                                  <Label htmlFor="name">Name</Label>
                                  <Input
                                    id="name"
                                    name="name"
                                    type="text"
                                    className="w-full"
                                    defaultValue={data.content.metadata.name}
                                  />
                                </div>
                                <div className="grid gap-3">
                                  <Label htmlFor="labels">Labels</Label>
                                  <Labels initialLabels={data.content.metadata.labels}/>
                                </div>
                                <div className="grid gap-3">
                                  <Label htmlFor="name">Language</Label>
                                  <Input
                                    id="languageTag"
                                    name="languageTag"
                                    type="text"
                                    className="w-full"
                                    defaultValue={data.content.metadata.languageTag}
                                  />
                                </div>
                                <div className="grid gap-3">
                                  <Label htmlFor="name">Content Type</Label>
                                  <Input
                                    id="contentType"
                                    name="contentType"
                                    type="text"
                                    className="w-full"
                                    defaultValue={data.content.metadata.content.type}
                                  />
                                </div>
                                <div className="grid gap-3">
                                  <Label htmlFor="name">Created</Label>
                                  {data.content.metadata.created}
                                </div>
                                <div className="grid gap-3">
                                  <Label htmlFor="name">Modified</Label>
                                  {data.content.metadata.modified}
                                </div>
                                <div className="grid gap-3">
                                  <Label htmlFor="name">Uploaded</Label>
                                  {data.content.metadata.uploaded || '--'}
                                </div>
                                <div className="grid gap-3">
                                  <Label htmlFor="name">Ready</Label>
                                  {data.content.metadata.ready || '--'}
                                </div>
                                <div className="grid grid-cols-2 gap-6">
                                  <ButtonLink
                                    href={'/metadata/' + data.content.metadata.id + '?delete=true'}
                                    variant="outline" size="sm">
                                    Delete
                                  </ButtonLink>
                                  <Button size="sm" type="submit">Save Metadata</Button>
                                </div>
                              </div>
                            </form>
                          </CardContent>
                        </Card>
                        <Card x-chunk="dashboard-07-chunk-0">
                          <CardHeader>
                            <CardTitle>Metadata Attributes</CardTitle>
                          </CardHeader>
                          <CardContent>
                            <JsonEditor
                              id={data.content.metadata.id}
                              attributes={data.content.metadata.attributes}
                              saveAttributes={setAttributes}/>
                          </CardContent>
                        </Card>
                        <Card x-chunk="dashboard-07-chunk-0">
                          <CardHeader>
                            <CardTitle>Metadata System Attributes</CardTitle>
                          </CardHeader>
                          <CardContent>
                            <JsonEditor
                              id={data.content.metadata.id}
                              attributes={data.content.metadata.systemAttributes}/>
                          </CardContent>
                        </Card>
                        <Card x-chunk="dashboard-07-chunk-0">
                          <CardHeader>
                            <CardTitle>Metadata Relationships</CardTitle>
                          </CardHeader>
                          <CardContent>
                            {
                              // eslint-disable-next-line @typescript-eslint/no-explicit-any
                              data.content.metadata.relationships.map((r: any, index: number) =>
                                <Relationship key={r.metadata.id} metadata={data.content.metadata} relationship={r} index={index} />
                              )
                            }
                          </CardContent>
                        </Card>
                      </div>
                      <div className="grid auto-rows-max items-start gap-4">
                        <Card x-chunk="dashboard-07-chunk-3">
                          <CardHeader>
                            <CardTitle>Metadata State</CardTitle>
                          </CardHeader>
                          <CardContent>
                            <div className="grid gap-6">
                              <div className="grid gap-3">
                                <States
                                  id={data.content.metadata.id}
                                  version={data.content.metadata.version}
                                  states={data.workflows.states.all}
                                  current={data.content.metadata.workflow.state}
                                  pending={data.content.metadata.workflow.pending}
                                />
                              </div>
                              {
                                data.content.metadata.ready ?
                                  <></> :
                                  <ButtonLink href={'/metadata/' + data.content.metadata.id + '?ready=true'}>Mark
                                    Ready</ButtonLink>
                              }
                            </div>
                          </CardContent>
                        </Card>
                        <Card x-chunk="dashboard-07-chunk-3">
                          <CardHeader>
                            <CardTitle>Metadata Permissions</CardTitle>
                          </CardHeader>
                          <CardContent>
                            Is Public?: <a
                              href={'/metadata/' + data.content.metadata.id + '?set-public=' + (!data.content.metadata['public'])}>{data.content.metadata['public'].toString()}</a><br />
                            Is Public Content?: <a
                              href={'/metadata/' + data.content.metadata.id + '?set-public-content=' + (!data.content.metadata['publicContent'])}>{data.content.metadata['publicContent'].toString()}</a><br />
                            Is Public Supplementary?: <a
                              href={'/metadata/' + data.content.metadata.id + '?set-public-supplementary=' + (!data.content.metadata['publicSupplementary'])}>{data.content.metadata['publicSupplementary'].toString()}</a>
                            <table className="mt-2">
                              <tbody>
                                {
                                  // eslint-disable-next-line @typescript-eslint/no-explicit-any
                                  data.content.metadata.permissions.map((p: any) => {
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
                        <Card
                          className="overflow-hidden" x-chunk="dashboard-07-chunk-4"
                        >
                          <CardHeader>
                            <CardTitle>Traits</CardTitle>
                            <CardDescription>
                              Metadata Traits
                            </CardDescription>
                          </CardHeader>
                          <CardContent>
                            <div className="grid gap-3">
                              <Traits id={data.content.metadata.id} traits={data.content.traits}
                                current={data.content.metadata.traitIds}/>
                            </div>
                            <table className="mt-2">
                              <tbody>
                                {
                                  // eslint-disable-next-line @typescript-eslint/no-explicit-any
                                  data.content.metadata.traitIds.map((trait: any) => {
                                    // eslint-disable-next-line @typescript-eslint/no-explicit-any
                                    const t = data.content.traits.filter((t: any) => t.id == trait)[0]
                                    return (
                                      <tr key={t.id}>
                                        <td className="w-64">{t.name || t.id}</td>
                                        <td><ButtonLink
                                          href={'/metadata/' + data.content.metadata.id + '?delete-trait=' + t.id}
                                          variant="outline" size="sm">Delete</ButtonLink></td>
                                      </tr>
                                    )
                                  })
                                }
                              </tbody>
                            </table>
                          </CardContent>
                        </Card>
                        <Card
                          className="overflow-hidden" x-chunk="dashboard-07-chunk-4"
                        >
                          <CardHeader>
                            <CardTitle>Workflows</CardTitle>
                            <CardDescription>
                              Metadata Workflows
                            </CardDescription>
                          </CardHeader>
                          <CardContent>
                            <Workflows workflows={data.workflows.all} id={data.content.metadata.id} />
                            <table>
                              <tbody>
                                <tr>
                                  <td className="p-2">ID</td>
                                  <td className="p-2">Workflow</td>
                                  <td className="p-2">Pending / Running / Complete</td>
                                </tr>
                                {
                                  // eslint-disable-next-line @typescript-eslint/no-explicit-any
                                  data.content.metadata.workflow.plans.map((plan: any) =>
                                    <tr key={plan.id}>
                                      <td className="p-2">{plan.id}</td>
                                      <td className="p-2">{plan.workflow.name || plan.workflow.id}</td>
                                      <td className="p-2">{plan.pending.length} / {plan.running.length} / {plan.complete.length}</td>
                                    </tr>
                                  )
                                }
                              </tbody>
                            </table>
                          </CardContent>
                        </Card>
                        {data.content.metadata.uploaded ?
                          <Card
                            className="overflow-hidden" x-chunk="dashboard-07-chunk-4"
                          >
                            <CardHeader>
                              <CardTitle>Contents</CardTitle>
                              <CardDescription>
                                Manage Content
                              </CardDescription>
                            </CardHeader>
                            <CardContent>
                              <div className="grid grid-cols-2 gap-6">
                                <ButtonLink href={data.content.metadata.content.urls.download.url}
                                  size="sm">Download</ButtonLink>
                                <ButtonLink href={'/metadata/' + data.content.metadata.id + '?delete-content=true'}
                                  variant="outline" size="sm">Delete</ButtonLink>
                              </div>
                            </CardContent>
                          </Card> :
                          <Card
                            className="overflow-hidden" x-chunk="dashboard-07-chunk-4"
                          >
                            <CardHeader>
                              <CardTitle>Upload</CardTitle>
                              <CardDescription>
                                Upload Content
                              </CardDescription>
                            </CardHeader>
                            <CardContent>
                              <UploadForm id={data.content.metadata.id}/>
                            </CardContent>
                          </Card>
                        }
                        <Card x-chunk="dashboard-07-chunk-0">
                          <CardHeader>
                            <CardTitle>Metadata Supplementary</CardTitle>
                          </CardHeader>
                          <CardContent>
                            <table>
                              <tbody>
                                {
                                  // eslint-disable-next-line @typescript-eslint/no-explicit-any
                                  data.content.metadata.supplementary.map((supplementary: any) =>
                                    <tr key={supplementary.key}>
                                      <td className="p-2">{supplementary.key}</td>
                                      <td className="p-2">{supplementary.name}</td>
                                      <td>
                                        <ButtonLink
                                          href={supplementary.content.urls.download.url}
                                          variant="outline" size="sm">Download</ButtonLink>
                                      </td>
                                    </tr>
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
                                  data.content.metadata.parentCollections.map((collection: any) =>
                                    <tr key={collection.id}>
                                      <td className="p-2"><a
                                        href={'/collections/' + collection.id}>{collection.name}</a></td>
                                      <td>
                                        <ButtonLink
                                          href={'/metadata/' + data.content.metadata.id + '?remove-collection=' + collection.id}
                                          variant="outline" size="sm">Remove</ButtonLink>
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
      <CommandMenu actions={[]} current={(data && data.content && data.content.metadata) ? { id: params.id, item: data.content.metadata, collection: false } : null}  />
    </>
  )
}
