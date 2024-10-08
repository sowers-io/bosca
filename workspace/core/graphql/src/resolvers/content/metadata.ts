/*
 * Copyright 2024 Sowers, LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import {
  Metadata as GMetadata,
  Resolvers,
  SignedUrl as GSignedUrl,
  Supplementary,
  MetadataWorkflowJob,
  SupplementaryContent,
  SupplementaryUrls,
  MetadataUrls,
  MetadataContent,
} from '../../generated/resolvers'
import {
  executeGraphQL,
  executeHttpRequest,
  getGraphQLHeaders,
  GraphQLRequestContext,
  toArrayBuffer,
  useClient,
  useServiceAccountClient,
} from '@bosca/common'
import {
  AddMetadataRequest,
  ContentService,
  IdRequest,
  Metadata,
  MetadataReadyRequest,
  SignedUrl,
  SupplementaryIdRequest,
  WorkflowQueueService,
} from '@bosca/protobufs'
import { protoInt64 } from '@bufbuild/protobuf'
import { GraphQLError } from 'graphql'
import { toGraphPermissions, toGrpcPermissions } from '../../util'

export function transformMetadata(metadata: Metadata): GMetadata {
  const m = metadata.toJson() as unknown as GMetadata
  m.__typename = 'Metadata'
  if (metadata.attributes) {
    m.attributes = []
    for (const key in metadata.attributes) {
      m.attributes.push({
        key: key,
        value: metadata.attributes[key],
      })
    }
  }
  m.workflowState = {
    id: metadata.workflowStateId,
    pendingId: metadata.workflowStatePendingId,
    deleteWorkflowId: metadata.deleteWorkflowId,
  }
  return m
}

export const resolvers: Resolvers<GraphQLRequestContext> = {
  Query: {
    metadata: async (_, args, context) => {
      return await executeGraphQL(async () => {
        const service = useClient(ContentService)
        const metadata = await service.getMetadata(new IdRequest({ id: args.id }), {
          headers: await getGraphQLHeaders(context),
        })
        return transformMetadata(metadata)
      })
    },
  },
  Metadata: {
    workflowJobs: async (parent, _, context) => {
      return await executeGraphQL<MetadataWorkflowJob[]>(async () => {
        const service = useClient(ContentService)
        const queueService = useServiceAccountClient(WorkflowQueueService)
        const jobIds = await service.getMetadataWorkflowJobs(new IdRequest({ id: parent.id }), {
          headers: await getGraphQLHeaders(context),
        })
        const jobs: MetadataWorkflowJob[] = []
        for (const jobId of jobIds.ids) {
          const job = await queueService.getJob(jobId)
          jobs.push({
            id: jobId.id,
            queue: jobId.queue,
            json: JSON.parse(job.json),
          })
        }
        return jobs
      })
    },
    supplementary: async (parent, args, context) => {
      return (await executeGraphQL(async () => {
        const service = useClient(ContentService)
        const request = new SupplementaryIdRequest({ id: parent.id, key: args.key })
        const response = await service.getMetadataSupplementary(request, {
          headers: await getGraphQLHeaders(context),
        })
        return response?.toJson() as unknown as Supplementary
      }))
    },
    supplementaries: async (parent, args, context) => {
      return (await executeGraphQL(async () => {
        const service = useClient(ContentService)
        const request = new IdRequest({ id: parent.id })
        const response = await service.getMetadataSupplementaries(request, {
          headers: await getGraphQLHeaders(context),
        })
        return response
          .supplementaries
          .filter((s) => !args.key || args.key?.includes(s.key))
          .map((s) => s.toJson()) as unknown as Supplementary[]
      }))!
    },
    permissions: async (parent, _, context) => {
      return (await executeGraphQL(async () => {
        const service = useClient(ContentService)
        const request = new IdRequest({ id: parent.id })
        const response = await service.getMetadataPermissions(request, {
          headers: await getGraphQLHeaders(context),
        })
        return toGraphPermissions(parent.id, response)
      }))!
    },
    content: async (parent, _, context) => {
      const type = parent.contentType.split(';')[0].trim()
      const urls: MetadataUrls = {
        id: parent.id!,
      }
      const result: MetadataContent = {
        urls: urls,
        json: null,
        text: null,
      }
      if (type === 'text/plain' || type === 'text/json') {
        const url: SignedUrl = await executeGraphQL(async () => {
          const service = useClient(ContentService)
          const request = new IdRequest({ id: parent.id })
          return await service.getMetadataDownloadUrl(request, {
            headers: await getGraphQLHeaders(context),
          })
        })
        if (!url) return result
        urls.download = url.toJson() as unknown as GSignedUrl
        if (type === 'text/json') {
          const content = await executeHttpRequest(url)
          result.json = JSON.parse(content.toString())
        } else if (type === 'text/plain') {
          const content = await executeHttpRequest(url)
          result.text = content.toString()
        }
        return result
      }
      return result
    },
  },
  MetadataUrls: {
    upload: async (parent, _, context) => {
      return await executeGraphQL<GSignedUrl>(async () => {
        const service = useClient(ContentService)
        const url = await service.getMetadataUploadUrl(new IdRequest({ id: parent.id }), {
          headers: await getGraphQLHeaders(context),
        })
        return url.toJson() as unknown as GSignedUrl
      })
    },
    download: async (parent, _, context) => {
      if (parent.download) return parent.download
      return (await executeGraphQL(async () => {
        const service = useClient(ContentService)
        const url = await service.getMetadataDownloadUrl(new IdRequest({ id: parent.id }), {
          headers: await getGraphQLHeaders(context),
        })
        return url.toJson() as unknown as GSignedUrl
      }))!
    },
  },
  SupplementaryUrls: {
    download: async (parent, _, context) => {
      if (parent.download) return parent.download
      return await executeGraphQL(async () => {
        const service = useClient(ContentService)
        const request = new SupplementaryIdRequest({ id: parent.id, key: parent.key })
        const url = await service.getMetadataSupplementaryDownloadUrl(request, {
          headers: await getGraphQLHeaders(context),
        })
        return url.toJson() as unknown as GSignedUrl
      })
    },
  },
  Supplementary: {
    content: async (parent, _, context) => {
      const type = parent.contentType.split(';')[0].trim()
      const urls: SupplementaryUrls = {
        id: parent.metadataId!,
        key: parent.key!,
      }
      const result: SupplementaryContent = {
        urls: urls,
        json: null,
        text: null,
      }
      if (type === 'text/plain' || type === 'text/json') {
        const url: SignedUrl = await executeGraphQL(async () => {
          const service = useClient(ContentService)
          const request = new SupplementaryIdRequest({ id: parent.metadataId, key: parent.key })
          return await service.getMetadataSupplementaryDownloadUrl(request, {
            headers: await getGraphQLHeaders(context),
          })
        })
        if (!url) return result
        urls.download = url.toJson() as unknown as GSignedUrl
        if (type === 'text/json') {
          const content = await executeHttpRequest(url)
          result.json = JSON.parse(content.toString())
        } else if (type === 'text/plain') {
          const content = await executeHttpRequest(url)
          result.text = content.toString()
        }
        return result
      }
      return result
    },
  },
  Mutation: {
    addMetadata: async (_, args, context) => {
      return await executeGraphQL(async () => {
        const service = useClient(ContentService)
        const response = await service.addMetadata(
          new AddMetadataRequest({
            collection: args.parent || '00000000-0000-0000-0000-000000000000',
            metadata: {
              name: args.metadata.name,
              contentType: args.metadata.contentType,
              traitIds: args.metadata.traitIds?.map((s) => s),
              contentLength: args.metadata.contentLength ? protoInt64.parse(args.metadata.contentLength) : undefined,
              languageTag: args.metadata.languageTag,
            },
          }), {
            headers: await getGraphQLHeaders(context),
          },
        )
        let lastError: any | null = null
        for (let tries = 0; tries < 100; tries++) {
          try {
            const metadata = await service.getMetadata(new IdRequest({ id: response.id }), {
              headers: await getGraphQLHeaders(context),
            })
            return transformMetadata(metadata)
          } catch (e) {
            lastError = e
            await new Promise((resolve) => setTimeout(resolve, 100))
          }
        }
        if (lastError) {
          throw lastError
        }
        throw new GraphQLError('failed to get metadata after it was created')
      })
    },
    setMetadataReady: async (_, args, context) => {
      return await executeGraphQL(async () => {
        const service = useClient(ContentService)
        await service.setMetadataReady(new MetadataReadyRequest({ id: args.id }), {
          headers: await getGraphQLHeaders(context),
        })
        const metadata = await service.getMetadata(new IdRequest({ id: args.id }), {
          headers: await getGraphQLHeaders(context),
        })
        return transformMetadata(metadata)
      })
    },
    addMetadataPermissions: async (_, args, context) => {
      return await executeGraphQL(async () => {
        const service = useClient(ContentService)
        await service.addMetadataPermissions(toGrpcPermissions(args.id, args.permissions), {
          headers: await getGraphQLHeaders(context),
        })
        const metadata = await service.getMetadata(new IdRequest({ id: args.id }), {
          headers: await getGraphQLHeaders(context),
        })
        return transformMetadata(metadata)
      })
    },
    deleteMetadataPermissions: async (_, args, context) => {
      return await executeGraphQL(async () => {
        const service = useClient(ContentService)
        await service.deleteMetadataPermissions(toGrpcPermissions(args.id, args.permissions), {
          headers: await getGraphQLHeaders(context),
        })
        const metadata = await service.getMetadata(new IdRequest({ id: args.id }), {
          headers: await getGraphQLHeaders(context),
        })
        return transformMetadata(metadata)
      })
    },
    deleteMetadata: async (_, args, context) => {
      return await executeGraphQL(async () => {
        const service = useClient(ContentService)
        await service.deleteMetadata(new IdRequest({ id: args.id! }), {
          headers: await getGraphQLHeaders(context),
        })
        return true
      })
    },
    setMetadataJSONContent: async (_, args, context) => {
      if (!args.json) throw new GraphQLError('missing json')
      return await executeGraphQL(async () => {
        const service = useClient(ContentService)
        const idRequest = new IdRequest({ id: args.id })
        const url = await service.getMetadataUploadUrl(idRequest, {
          headers: await getGraphQLHeaders(context),
        })
        const content = typeof args.json === 'string' ? args.json : JSON.stringify(args.json)
        await executeHttpRequest(url, toArrayBuffer(content))
        const metadata = await service.getMetadata(idRequest, {
          headers: await getGraphQLHeaders(context),
        })
        return transformMetadata(metadata)
      })
    },
    setMetadataTextContent: async (_, args, context) => {
      if (!args.text) throw new GraphQLError('missing json')
      return await executeGraphQL(async () => {
        const service = useClient(ContentService)
        const idRequest = new IdRequest({ id: args.id })
        const url = await service.getMetadataUploadUrl(idRequest, {
          headers: await getGraphQLHeaders(context),
        })
        await executeHttpRequest(url, toArrayBuffer(args.text!))
        const metadata = await service.getMetadata(idRequest, {
          headers: await getGraphQLHeaders(context),
        })
        return transformMetadata(metadata)
      })
    },
  },
}
