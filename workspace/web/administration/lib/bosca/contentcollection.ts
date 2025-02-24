import { Api } from './api'
import {
  AddCollectionCollectionDocument,
  AddCollectionDocument,
  AddMetadataCollectionDocument,
  BeginCollectionTransitionDocument,
  type CollectionFragment,
  type CollectionInput,
  type CollectionMetadataRelationshipFragment,
  DeleteCollectionDocument,
  ExtensionFilterType,
  type FindAttributes,
  FindCollectionDocument,
  GetCollectionDocument,
  GetCollectionListDocument,
  GetCollectionMetadataRelationshipsDocument,
  GetCollectionParentsDocument,
  type MetadataFragment,
  type ParentCollectionFragment,
  RemoveCollectionCollectionDocument,
  RemoveMetadataCollectionDocument,
  SetCollectionPublicDocument,
  SetCollectionPublicListDocument,
  SetCollectionReadyDocument,
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
    categoryIds?: Array<string[]> | Ref<Array<string[]>> | null
    offset?: number | Ref<number>
    limit?: number | Ref<number>
  }): AsyncData<CollectionFragment[] | null, any> {
    return this.executeAndTransformAsyncData(
      FindCollectionDocument,
      {
        query: {
          attributes: query.attributes,
          extension: query.extension,
          categoryIds: query.categoryIds,
          offset: query.offset,
          limit: query.limit,
        }
      },
      (data) => {
        if (!data) return null
        return data.content.findCollection as CollectionFragment[]
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

  async getRelationships(id: string): Promise<CollectionMetadataRelationshipFragment[] | null> {
    const response = await this.network.execute(GetCollectionMetadataRelationshipsDocument, {
      id: id,
    })
    return response?.content?.collection?.metadataRelationships as CollectionMetadataRelationshipFragment[] | null
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

  async delete(id: string): Promise<void> {
    await this.network.execute(DeleteCollectionDocument, { id })
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
    const response = await this.network.execute(FindCollectionDocument, {
      query: {
        attributes,
        offset,
        limit,
      }
    })
    return response!.content!.findCollection as CollectionFragment[]
  }

  async beginTransition(id: string, state: string) {
    await this.network.execute(BeginCollectionTransitionDocument, {
      id,
      state,
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
}
