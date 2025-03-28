import { Api } from './api'
import {
  AddCollectionCollectionDocument,
  AddCollectionDocument,
  AddCollectionMetadataRelationshipDocument,
  AddMetadataCollectionDocument,
  BeginCollectionTransitionDocument,
  type CollectionFragment,
  type CollectionInput,
  type CollectionMetadataRelationshipFragment,
  type CollectionMetadataRelationshipInput,
  type CollectionSupplementaryFragment,
  CollectionType,
  DeleteCollectionDocument,
  EditCollectionDocument,
  EnqueueWorkflowDocument,
  ExtensionFilterType,
  type FindAttributes,
  FindCollectionsCountDocument,
  FindCollectionsDocument,
  GetCollectionChildrenCollectionsDocument,
  GetCollectionChildrenMetadataDocument,
  GetCollectionDocument,
  GetCollectionListDocument,
  GetCollectionMetadataRelationshipsDocument,
  GetCollectionParentsDocument,
  GetCollectionSupplementaryDocument,
  GetMetadataSupplementaryDocument,
  type MetadataFragment,
  type ParentCollectionFragment,
  RemoveCollectionCollectionDocument,
  RemoveCollectionMetadataRelationshipDocument,
  RemoveMetadataCollectionDocument,
  SetCollectionAttributesDocument,
  SetCollectionPublicDocument,
  SetCollectionPublicListDocument,
  SetCollectionReadyDocument,
  type WorkflowConfigurationInput,
  type WorkflowExecutionId,
} from '~/lib/graphql/graphql'
import type { AsyncData } from '#app/composables/asyncData'
import { NetworkClient } from '~/lib/bosca/networkclient'

export type CollectionItem = CollectionFragment | MetadataFragment

export interface CollectionAndItems {
  collection: CollectionFragment
  items: Array<CollectionItem>
}

export class ContentCollections<T extends NetworkClient> extends Api<T> {
  constructor(network: T) {
    super(network)
  }

  async getCollectionChildCollections(
    id: string,
    offset: number | Ref<number>,
    limit: number | Ref<number>,
  ): Promise<{ collections: Array<CollectionFragment>; count: number }> {
    const response = await this.network.execute(
      GetCollectionChildrenCollectionsDocument,
      {
        id,
        offset,
        limit,
      },
    )
    if (response?.content?.collection?.collections) {
      return {
        collections: response?.content?.collection?.collections as Array<
          CollectionFragment
        >,
        count: response?.content?.collection?.collectionsCount || 0,
      }
    }
    return { collections: [], count: 0 }
  }

  async getCollectionChildMetadata(
    id: string,
    offset: number | Ref<number>,
    limit: number | Ref<number>,
  ): Promise<{ metadata: Array<MetadataFragment>; count: number }> {
    const response = await this.network.execute(
      GetCollectionChildrenMetadataDocument,
      {
        id,
        offset,
        limit,
      },
    )
    if (response?.content?.collection?.metadata) {
      return {
        metadata: response?.content?.collection?.metadata as Array<
          MetadataFragment
        >,
        count: response?.content?.collection?.metadataCount || 0,
      }
    }
    return { metadata: [], count: 0 }
  }

  getCollectionChildMetadataAsyncData(
    id: string,
    offset: number | Ref<number>,
    limit: number | Ref<number>,
  ): AsyncData<
    { metadata: Array<MetadataFragment>; count: number } | null,
    any
  > {
    return this.executeAndTransformAsyncData(
      GetCollectionChildrenMetadataDocument,
      {
        id,
        offset,
        limit,
      },
      (data) => {
        if (!data) return null
        return {
          metadata: data?.content?.collection?.metadata as Array<
            MetadataFragment
          >,
          count: data?.content?.collection?.metadataCount || 0,
        }
      },
    )
  }

  async getCollectionParents(
    id: string,
  ): Promise<Array<ParentCollectionFragment> | null> {
    const response = await this.network.execute(GetCollectionParentsDocument, {
      id,
    })
    return response?.content?.collection?.parentCollections as Array<
      ParentCollectionFragment
    >
  }

  getCollectionParentAsyncData(
    id: string | Ref<string, string>,
  ): AsyncData<ParentCollectionFragment | null, any> {
    return this.executeAndTransformAsyncData(
      GetCollectionParentsDocument,
      { id },
      (data) => {
        if (!data) return null
        return data.content?.collection
          ?.parentCollections[0] as ParentCollectionFragment
      },
    )
  }

  findAsyncData(query: {
    attributes?:
      | Array<FindAttributes>
      | Ref<Array<FindAttributes>>
      | null
    extension?: ExtensionFilterType | Ref<ExtensionFilterType> | null
    categoryIds?: Array<string> | Ref<Array<string>> | null
    type?: CollectionType | Ref<CollectionType> | null
    offset?: number | Ref<number>
    limit?: number | Ref<number>
  }): AsyncData<CollectionFragment[] | null, any> {
    const q = computed(() => {
      return {
        attributes: unref(query.attributes),
        extension: unref(query.extension),
        categoryIds: unref(query.categoryIds),
        // @ts-ignore: this should be fine
        contentTypes: unref(query.contentTypes),
        collectionType: unref(query.type),
        offset: unref(query.offset),
        limit: unref(query.limit),
      }
    })
    return this.executeAndTransformAsyncData(
      FindCollectionsDocument,
      { query: q },
      (data) => {
        if (!data) return null
        return data.content.findCollections as CollectionFragment[]
      },
    )
  }

  findCountAsyncData(query: {
    attributes?:
      | Array<FindAttributes>
      | Ref<Array<FindAttributes>>
      | null
    extension?: ExtensionFilterType | Ref<ExtensionFilterType> | null
    categoryIds?: Array<string> | Ref<Array<string>> | null
    type?: CollectionType | Ref<CollectionType> | null
    offset?: number | Ref<number>
    limit?: number | Ref<number>
  }): AsyncData<number | null, any> {
    const q = computed(() => {
      return {
        attributes: unref(query.attributes),
        extension: unref(query.extension),
        categoryIds: unref(query.categoryIds),
        // @ts-ignore: this should be fine
        contentTypes: unref(query.contentTypes),
        collectionType: unref(query.type),
        offset: unref(query.offset),
        limit: unref(query.limit),
      }
    })
    return this.executeAndTransformAsyncData(
      FindCollectionsCountDocument,
      { query: q },
      (data) => {
        if (!data) return null
        return data.content.findCollectionsCount || 0
      },
    )
  }

  async list(id: string): Promise<CollectionAndItems | null> {
    const response = await this.network.execute(GetCollectionListDocument, {
      id,
    })
    if (!response?.content?.collection) return null
    return {
      collection: response!.content!.collection!,
      items: response!.content!.collection!.items,
    } as CollectionAndItems
  }

  listAsyncData(
    id: string | Ref<string, string>,
  ): AsyncData<CollectionAndItems | null, any> {
    return this.executeAndTransformAsyncData(
      GetCollectionListDocument,
      { id },
      (data) => {
        console.error(data)
        if (!data) return null
        return {
          collection: data!.content!.collection!,
          items: data!.content!.collection!.items,
        } as CollectionAndItems
      },
    )
  }

  async get(id: string): Promise<CollectionFragment | null> {
    const response = await this.network.execute(GetCollectionDocument, {
      id: id,
    })
    return response?.content.collection as CollectionFragment | null
  }

  async getSupplementary(
    id: string,
    key: string,
  ): Promise<CollectionSupplementaryFragment | null> {
    const response = await this.network.execute(
      GetCollectionSupplementaryDocument,
      {
        id: id,
        key,
      },
    )
    const supplementary = response?.content?.collection?.supplementary?.find(
      (s) => s.key === key,
    )
    return supplementary as CollectionSupplementaryFragment | null
  }

  async getMetadataRelationships(
    id: string,
  ): Promise<CollectionMetadataRelationshipFragment[] | null> {
    const response = await this.network.execute(
      GetCollectionMetadataRelationshipsDocument,
      {
        id: id,
      },
    )
    return response?.content?.collection?.metadataRelationships as
      | CollectionMetadataRelationshipFragment[]
      | null
  }

  getAsyncData(
    id: string | Ref<string, string>,
  ): AsyncData<CollectionFragment | null, any> {
    return this.executeAndTransformAsyncData(
      GetCollectionDocument,
      { id },
      (data) => {
        if (!data) return null
        return data.content.collection as CollectionFragment
      },
    )
  }

  // suspend fun getParents(id: String): List<ParentCollection> {
  //     val response = network.client.query(GetCollectionParentsQuery(Optional.presentIfNotNull(id))).execute()
  //     response.validate()
  //     return response.data?.content?.collection?.collectionParents?.parentCollections?.map { it.parentCollection } ?: emptyList()
  // }
  //
  // suspend fun getPermissions(id: String): List<Permission> {
  //     val response = network.client.query(GetCollectionPermissionsQuery(Optional.presentIfNotNull(id))).execute()
  //     response.validate()
  //     return response.data?.content?.collection?.permissions?.map { it.permission } ?: emptyList()
  // }

  async add(collection: CollectionInput): Promise<string> {
    const response = await this.network.execute(AddCollectionDocument, {
      collection,
    })
    return response!.content.collection.add.id
  }

  async edit(id: string, collection: CollectionInput): Promise<string> {
    const response = await this.network.execute(EditCollectionDocument, {
      id,
      input: collection,
    })
    return response!.content.collection.edit.id
  }

  async delete(id: string): Promise<void> {
    await this.network.execute(DeleteCollectionDocument, { id })
  }

  async setAttributes(id: string, attributes: any): Promise<void> {
    await this.network.execute(SetCollectionAttributesDocument, {
      id,
      attributes,
    })
  }

  // suspend fun edit(id: String, collection: CollectionInput): String? {
  //     val response = network.client.mutation(EditCollectionMutation(id, collection)).execute()
  //     response.validate()
  //     return response.data?.content?.collection?.edit?.id
  // }
  //
  // suspend fun setAttributes(id: String, attributes: Any?) {
  //     val response = network.client.mutation(SetCollectionAttributesMutation(id, attributes ?: emptyMap<Any, Any>())).execute()
  //     response.validate()
  // }
  //
  // suspend fun addPermission(input: PermissionInput) {
  //     val response = network.client.mutation(AddCollectionPermissionMutation(input)).execute()
  //     response.validate()
  // }
  //
  // suspend fun removePermission(input: PermissionInput) {
  //     val response = network.client.mutation(RemoveCollectionPermissionMutation(input)).execute()
  //     response.validate()
  // }
  //
  // suspend fun beginTransition(id: String, state: String): Boolean? {
  //     val response = network.client.mutation(BeginCollectionTransitionMutation(id, state)).execute()
  //     response.validate()
  //     return response.data?.workflows?.beginTransition
  // }
  //

  async findCollection(
    attributes: Array<FindAttributes>,
    offset: number | Ref<number>,
    limit: number | Ref<number>,
  ): Promise<CollectionFragment[]> {
    const response = await this.network.execute(FindCollectionsDocument, {
      query: {
        attributes,
        offset,
        limit,
      },
    })
    return response!.content!.findCollections as CollectionFragment[]
  }

  async beginTransition(id: string, state: string, status: string) {
    await this.network.execute(BeginCollectionTransitionDocument, {
      id,
      state,
      status,
    })
  }

  async setReady(id: string): Promise<void> {
    await this.network.execute(SetCollectionReadyDocument, { id })
  }

  async setPublic(id: string, isPublic: boolean): Promise<void> {
    await this.network.execute(SetCollectionPublicDocument, {
      id,
      public: isPublic,
    })
  }

  async setPublicList(id: string, isPublic: boolean): Promise<void> {
    await this.network.execute(SetCollectionPublicListDocument, {
      id,
      public: isPublic,
    })
  }

  async addCollection(collectionId: string, id: string): Promise<void> {
    await this.network.execute(AddCollectionCollectionDocument, {
      collectionId,
      id,
    })
  }

  async removeCollection(collectionId: string, id: string): Promise<void> {
    await this.network.execute(RemoveCollectionCollectionDocument, {
      collectionId,
      id,
    })
  }

  async addMetadata(collectionId: string, metadataId: string): Promise<void> {
    await this.network.execute(AddMetadataCollectionDocument, {
      collectionId,
      id: metadataId,
    })
  }

  async removeMetadata(
    collectionId: string,
    metadataId: string,
  ): Promise<void> {
    await this.network.execute(RemoveMetadataCollectionDocument, {
      collectionId,
      id: metadataId,
    })
  }

  async addMetadataRelationship(
    relationship: CollectionMetadataRelationshipInput,
  ): Promise<void> {
    await this.network.execute(AddCollectionMetadataRelationshipDocument, {
      relationship,
    })
  }

  async removeMetadataRelationship(
    id: string,
    metadataId: string,
    relationship: string,
  ): Promise<void> {
    await this.network.execute(RemoveCollectionMetadataRelationshipDocument, {
      id,
      metadataId,
      relationship,
    })
  }

  async enqueueCollectionWorkflow(
    workflowId: string,
    collectionId: string,
    configuration: Array<WorkflowConfigurationInput> = [],
  ): Promise<WorkflowExecutionId> {
    const response = await this.network.execute(EnqueueWorkflowDocument, {
      workflowId,
      collectionId,
      configuration,
    })
    return response!.workflows!.enqueueWorkflow!
  }
}
