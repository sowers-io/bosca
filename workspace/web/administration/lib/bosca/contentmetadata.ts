import { Api } from '~/lib/bosca/api'
import { NetworkClient } from '~/lib/bosca/networkclient'
import {
  AddDocumentDocument,
  AddGuideDocument,
  AddGuideStepDocument,
  AddMetadataDocument,
  AddMetadataPermissionDocument,
  AddMetadataRelationshipDocument,
  AddMetadataTraitDocument,
  BeginMetadataTransitionDocument,
  CancelTransitionDocument,
  type CollectionTemplateFragment,
  DeleteGuideDocument,
  DeleteGuideStepDocument,
  DeleteMetadataDocument,
  type DocumentFragment,
  type DocumentTemplateFragment,
  EditMetadataDocument,
  ExtensionFilterType,
  type FindAttributes,
  FindMetadataCountDocument,
  FindMetadataDocument,
  GetCollectionTemplateDocument,
  GetMetadataDocument,
  GetMetadataDocumentDocument,
  GetMetadataDocumentTemplateDocument,
  GetMetadataGuideDocument,
  GetMetadataGuideTemplateDocument,
  GetMetadataParentsDocument,
  GetMetadataPermissionsDocument,
  GetMetadataRelationshipsDocument,
  GetMetadataSupplementaryDocument,
  GetMetadataSupplementaryJsonDocument,
  GetMetadataSupplementaryTextDocument,
  GetMetadataUploadDocument,
  GetRunningWorkflowCountDocument,
  type GuideFragment,
  type GuideTemplateFragment,
  type MetadataFragment,
  type MetadataInput,
  type MetadataRelationshipFragment,
  type MetadataRelationshipInput,
  type MetadataSupplementaryFragment,
  type Ordering,
  type ParentCollectionFragment,
  type PermissionFragment,
  type PermissionInput,
  RemoveMetadataPermissionDocument,
  RemoveMetadataRelationshipDocument,
  RemoveMetadataTraitDocument,
  SetCollectionAttributesDocument,
  SetJsonContentsDocument,
  SetMetadataAttributesDocument,
  SetMetadataContentPublicDocument,
  SetMetadataPublicDocument,
  SetMetadataReadyDocument,
  SetMetadataSupplementaryPublicDocument,
  SetTextContentsDocument,
  type SignedUrl,
} from '~/lib/graphql/graphql'
import type { AsyncData } from '#app/composables/asyncData'
import type { Reactive } from 'vue'

export interface ContentTypeFilter {
  jpg: boolean
  png: boolean
  webp: boolean
  webm: boolean
  mp4: boolean
  mp3: boolean
  youtube: boolean
}

export class ContentMetadata<T extends NetworkClient> extends Api<T> {
  constructor(network: T) {
    super(network)
  }

  async get(id: string): Promise<MetadataFragment> {
    const response = await this.network.execute(GetMetadataDocument, {
      id: id,
    })
    return response!.content!.metadata!
  }

  async getRelationships(
    id: string,
  ): Promise<Array<MetadataRelationshipFragment>> {
    const response = await this.network.execute(
      GetMetadataRelationshipsDocument,
      {
        id: id,
      },
    )
    return response!.content!.metadata!.relationships as Array<
      MetadataRelationshipFragment
    >
  }

  async getGuide(id: string): Promise<GuideFragment> {
    const response = await this.network.execute(GetMetadataGuideDocument, {
      id: id,
    })
    return response!.content!.metadata!.guide as GuideFragment
  }

  async getGuideTemplate(
    id: string,
    version: number,
  ): Promise<GuideTemplateFragment> {
    const response = await this.network.execute(
      GetMetadataGuideTemplateDocument,
      {
        id,
        version,
      },
    )
    return response!.content!.metadata!.guideTemplate as GuideTemplateFragment
  }

  async getDocument(id: string): Promise<DocumentFragment> {
    const response = await this.network.execute(GetMetadataDocumentDocument, {
      id: id,
    })
    return response!.content!.metadata!.document as DocumentFragment
  }

  async getDocumentTemplate(
    id: string,
    version: number,
  ): Promise<DocumentTemplateFragment> {
    const response = await this.network.execute(
      GetMetadataDocumentTemplateDocument,
      {
        id,
        version,
      },
    )
    return response!.content!.metadata!
      .documentTemplate as DocumentTemplateFragment
  }

  async getCollectionTemplate(
    id: string,
    version: number,
  ): Promise<CollectionTemplateFragment> {
    const response = await this.network.execute(
      GetCollectionTemplateDocument,
      {
        id,
        version,
      },
    )
    return response!.content!.metadata!
      .collectionTemplate as CollectionTemplateFragment
  }

  async getUploadUrl(id: string): Promise<SignedUrl> {
    const response = await this.network.execute(GetMetadataUploadDocument, {
      id: id,
    })
    return response!.content!.metadata!.content!.urls.upload
  }

  private getContentTypes(filter: Reactive<ContentTypeFilter>): string[] {
    const contentTypes: string[] = []
    if (filter.jpg) contentTypes.push('image/jpeg')
    if (filter.png) contentTypes.push('image/png')
    if (filter.webp) contentTypes.push('image/webp')
    if (filter.mp4) {
      contentTypes.push('video/mp4')
      contentTypes.push('video/mpeg')
    }
    if (filter.mp3) {
      contentTypes.push('audio/mp3')
      contentTypes.push('audio/mpeg')
    }
    if (filter.youtube) {
      contentTypes.push('bosca/x-youtube-video')
    }
    if (filter.webm) contentTypes.push('video/webm')
    return contentTypes
  }

  getByContentType(
    filter: Reactive<ContentTypeFilter>,
    offset: Ref<number>,
    limit: Ref<number>,
  ): AsyncData<MetadataFragment[] | null, any> {
    const self = this
    const query = computed(() => {
      return {
        attributes: [],
        contentTypes: self.getContentTypes(filter),
        offset: offset.value,
        limit: limit.value,
      }
    })
    return this.executeAndTransformAsyncData(
      FindMetadataDocument,
      { query: query },
      (data) => {
        if (!data) return null
        return data.content.findMetadata as MetadataFragment[]
      },
    )
  }

  getByContentTypeCount(
    filter: Reactive<ContentTypeFilter>,
  ): AsyncData<number | null, any> {
    const self = this
    const query = computed(() => {
      return {
        attributes: [],
        contentTypes: self.getContentTypes(filter),
      }
    })
    return this.executeAndTransformAsyncData(
      FindMetadataCountDocument,
      { query: query },
      (data) => {
        if (!data) return 0
        return data.content.findMetadataCount || 0
      },
    )
  }

  getAsyncData(
    id: string | Ref<string, string>,
  ): AsyncData<MetadataFragment | null, any> {
    return this.executeAndTransformAsyncData(
      GetMetadataDocument,
      { id },
      (data) => {
        if (!data) return null
        return data.content.metadata as MetadataFragment
      },
    )
  }

  async getParents(
    id: string,
  ): Promise<Array<ParentCollectionFragment> | null> {
    const response = await this.network.execute(GetMetadataParentsDocument, {
      id,
    })
    return response?.content?.metadata?.parentCollections as Array<
      ParentCollectionFragment
    >
  }

  getParentsAsyncData(
    id: string,
  ): AsyncData<Array<ParentCollectionFragment> | null, any> {
    return this.executeAndTransformAsyncData(
      GetMetadataParentsDocument,
      { id },
      (data) => {
        if (!data) return null
        return data.content.metadata!.parentCollections as Array<
          ParentCollectionFragment
        >
      },
    )
  }

  getPermissionsAsyncData(
    id: string,
  ): AsyncData<Array<PermissionFragment> | null, any> {
    return this.executeAndTransformAsyncData(
      GetMetadataPermissionsDocument,
      { id },
      (data) => {
        if (!data) return null
        return data.content.metadata!.permissions as Array<
          PermissionFragment
        >
      },
    )
  }

  // suspend fun getTextContents(id: String): String? {
  //     val response = network.client.query(GetTextContentsQuery(id)).execute()
  //     response.validate()
  //     return response.data?.content?.metadata?.content?.text
  // }
  //
  // suspend fun getSupplementaryTextContents(id: String, key: String): String? {
  //     val response = network.client.query(GetSupplementaryTextContentsQuery(id, key)).execute()
  //     response.validate()
  //     return response.data?.content?.metadata?.supplementary?.firstOrNull()?.content?.text
  // }
  //

  async getSupplementary(
    id: string,
    key: string,
  ): Promise<MetadataSupplementaryFragment | null> {
    const response = await this.network.execute(
      GetMetadataSupplementaryDocument,
      {
        id: id,
        key,
      },
    )
    const supplementary = response?.content?.metadata?.supplementary?.find(
      (s) => s.key === key,
    )
    return supplementary as MetadataSupplementaryFragment | null
  }

  async getSupplementaryText(
    id: string,
    key: string,
  ): Promise<string | null> {
    const response = await this.network.execute(
      GetMetadataSupplementaryTextDocument,
      {
        id: id,
        key,
      },
    )
    const supplementary = response?.content?.metadata?.supplementary
    if (!supplementary || supplementary.length === 0) return null
    return supplementary[0]?.content?.text as string | null
  }

  async getSupplementaryJson(
    id: string,
    key: string,
  ): Promise<any | null> {
    const response = await this.network.execute(
      GetMetadataSupplementaryJsonDocument,
      {
        id: id,
        key,
      },
    )
    const supplementary = response?.content?.metadata?.supplementary
    if (!supplementary || supplementary.length === 0) return null
    return supplementary[0]?.content?.json
  }

  async getRunningWorkflowCount(id: string): Promise<number> {
    try {
      const response = await this.network.execute(
        GetRunningWorkflowCountDocument,
        {
          id,
        },
      )
      return response!.content?.metadata?.workflow?.running || 0
    } catch (e) {
      return 0
    }
  }

  async add(metadata: MetadataInput): Promise<string> {
    const response = await this.network.execute(AddMetadataDocument, {
      metadata,
    })
    return response!.content.metadata.add.id
  }

  async addDocument(
    parentCollectionId: string,
    templateId: string,
    templateVersion: number,
  ): Promise<string> {
    const response = await this.network.execute(AddDocumentDocument, {
      parentCollectionId,
      templateId,
      templateVersion,
    })
    return response!.content.metadata.addDocument!.id
  }

  async addGuide(
    parentCollectionId: string,
    templateId: string,
    templateVersion: number,
  ): Promise<string> {
    const response = await this.network.execute(AddGuideDocument, {
      parentCollectionId,
      templateId,
      templateVersion,
    })
    return response!.content.metadata.addGuide!.id
  }

  async addGuideStep(
    metadataId: string,
    version: number,
    sort: number,
    templateId: string,
    templateVersion: number,
    templateStepId: number,
  ): Promise<string> {
    const response = await this.network.execute(AddGuideStepDocument, {
      metadataId,
      version,
      sort,
      templateId,
      templateVersion,
      templateStepId,
    })
    return response!.content.metadata.addGuideStep!.metadata?.id || ''
  }

  async deleteGuideStep(
    metadataId: string,
    version: number,
    stepId: number,
  ): Promise<void> {
    await this.network.execute(DeleteGuideStepDocument, {
      metadataId,
      version,
      stepId,
    })
  }

  async edit(id: string, metadata: MetadataInput): Promise<string> {
    const response = await this.network.execute(EditMetadataDocument, {
      id,
      metadata,
    })
    return response!.content.metadata.edit.id
  }

  async setAttributes(id: string, attributes: any) {
    await this.network.execute(SetMetadataAttributesDocument, {
      id,
      attributes,
    })
  }

  async setTextContent(id: string, contentType: string, content: string) {
    await this.network.execute(SetTextContentsDocument, {
      id,
      contentType,
      content,
    })
  }

  async setJsonContent(id: string, contentType: string, content: any) {
    await this.network.execute(SetJsonContentsDocument, {
      id,
      contentType,
      content,
    })
  }

  async delete(id: string): Promise<void> {
    await this.network.execute(DeleteMetadataDocument, { id })
  }

  async deleteGuide(id: string, version: number): Promise<void> {
    await this.network.execute(DeleteGuideDocument, {
      id,
      version,
    })
  }

  async find(query: {
    attributes?:
      | Array<FindAttributes>
      | Ref<Array<FindAttributes>>
      | null
    extension?: ExtensionFilterType | Ref<ExtensionFilterType> | null
    categoryIds?: Array<string> | Ref<string[]> | null
    contentTypes?: Array<string> | Ref<string[]> | null
    offset?: number | Ref<number>
    limit?: number | Ref<number>
  }): Promise<MetadataFragment[]> {
    const q = {
      attributes: unref(query.attributes),
      extension: unref(query.extension),
      categoryIds: unref(query.categoryIds),
      // @ts-ignore: this should be fine
      contentTypes: unref(query.contentTypes),
      offset: unref(query.offset),
      limit: unref(query.limit),
    }
    const response = await this.network.execute(
      FindMetadataDocument,
      {
        query: q,
      },
    )
    return response?.content?.findMetadata as MetadataFragment[] | null || []
  }

  findAsyncData(query: {
    attributes?:
      | Array<FindAttributes>
      | Ref<Array<FindAttributes>>
      | null
    extension?: ExtensionFilterType | Ref<ExtensionFilterType> | null
    categoryIds?: Array<string> | Ref<string[]> | null
    contentTypes?: Array<string> | Ref<string[]> | null
    ordering?: Ordering[] | Ref<Ordering[]> | null
    offset?: number | Ref<number>
    limit?: number | Ref<number>
  }): AsyncData<MetadataFragment[] | null, any> {
    const q = computed(() => {
      return {
        attributes: unref(query.attributes),
        extension: unref(query.extension),
        categoryIds: unref(query.categoryIds),
        ordering: unref(query.ordering),
        // @ts-ignore: this should be fine
        contentTypes: unref(query.contentTypes),
        offset: unref(query.offset),
        limit: unref(query.limit),
      }
    })
    return this.executeAndTransformAsyncData(
      FindMetadataDocument,
      {
        query: q,
      },
      (data) => {
        if (!data) return null
        return data.content.findMetadata as MetadataFragment[]
      },
    )
  }

  findCountAsyncData(query: {
    attributes?:
      | Array<FindAttributes>
      | Ref<Array<FindAttributes>>
      | null
    extension?: ExtensionFilterType | Ref<ExtensionFilterType> | null
    contentTypes?: Array<string> | Ref<string[]> | null
    categoryIds?: Array<string> | Ref<Array<string>> | null
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
        offset: unref(query.offset),
        limit: unref(query.limit),
      }
    })
    return this.executeAndTransformAsyncData(
      FindMetadataCountDocument,
      { query: q },
      (data) => {
        if (!data) return null
        return data.content.findMetadataCount || 0
      },
    )
  }

  async addFiles(
    parentCollectionId: string,
    files: File[],
    traitIds: string[][] = [],
  ): Promise<string[]> {
    const metadataIds: string[] = []
    for (let i = 0; i < files.length; i++) {
      const file = files[i]
      const metadataId = await this.add({
        parentCollectionId: parentCollectionId,
        name: file.name,
        languageTag: 'en',
        contentType: file.type,
        traitIds: traitIds[i] || [],
      })
      const url = await this.getUploadUrl(metadataId)
      const headers = new Headers()
      for (const hdr of url.headers) {
        headers.append(hdr.name, hdr.value)
      }
      const data = new FormData()
      data.append('file', file)
      const f = typeof $fetch === 'function' ? $fetch : fetch
      const response = await f(url.url, {
        method: 'POST',
        body: data,
        headers: headers,
      })
      if (response !== 'Upload successful') {
        throw new Error('Failed to upload file contents: ' + response)
      }
      metadataIds.push(metadataId)
    }
    return metadataIds
  }

  getRelationshipsAsyncData(
    id: string,
  ): AsyncData<Array<MetadataRelationshipFragment> | null, any> {
    return this.executeAndTransformAsyncData(
      GetMetadataRelationshipsDocument,
      { id },
      (data) => {
        if (!data) return null
        return data.content.metadata!.relationships as Array<
          MetadataRelationshipFragment
        >
      },
    )
  }

  async addRelationship(relationship: MetadataRelationshipInput) {
    await this.network.execute(AddMetadataRelationshipDocument, {
      relationship,
    })
  }

  async removeRelationship(id1: string, id2: string, relationship: string) {
    await this.network.execute(RemoveMetadataRelationshipDocument, {
      id1,
      id2,
      relationship,
    })
  }

  async addPermission(permission: PermissionInput) {
    await this.network.execute(AddMetadataPermissionDocument, { permission })
  }

  async removePermission(permission: PermissionInput) {
    await this.network.execute(RemoveMetadataPermissionDocument, { permission })
  }

  async beginTransition(
    id: string,
    version: number,
    state: string,
    status: string,
    stateValid: Date | null = null,
  ) {
    await this.network.execute(BeginMetadataTransitionDocument, {
      id,
      version,
      state,
      stateValid: stateValid ? stateValid.toISOString() : null,
      status,
    })
  }

  async cancelTransition(
    id: string,
    version: number,
  ) {
    await this.network.execute(CancelTransitionDocument, {
      metadataId: id,
      version,
    })
  }

  async setReady(id: string): Promise<void> {
    await this.network.execute(SetMetadataReadyDocument, { id })
  }

  async setPublic(id: string, isPublic: boolean): Promise<void> {
    await this.network.execute(SetMetadataPublicDocument, {
      id,
      public: isPublic,
    })
  }

  async setContentPublic(id: string, isPublic: boolean): Promise<void> {
    await this.network.execute(SetMetadataContentPublicDocument, {
      id,
      public: isPublic,
    })
  }

  async setSupplementaryPublic(id: string, isPublic: boolean): Promise<void> {
    await this.network.execute(SetMetadataSupplementaryPublicDocument, {
      id,
      public: isPublic,
    })
  }

  async addTrait(metadataId: string, traitId: string) {
    await this.network.execute(AddMetadataTraitDocument, {
      metadataId,
      traitId,
    })
  }

  async removeTrait(metadataId: string, traitId: string) {
    await this.network.execute(RemoveMetadataTraitDocument, {
      metadataId,
      traitId,
    })
  }
}
