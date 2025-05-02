package io.bosca.api

import com.apollographql.apollo.api.Upload
import com.apollographql.apollo.api.toUpload
import io.bosca.graphql.*
import io.bosca.graphql.fragment.*
import io.bosca.graphql.fragment.Category
import io.bosca.graphql.fragment.Document
import io.bosca.graphql.fragment.DocumentTemplate
import io.bosca.graphql.fragment.Guide
import io.bosca.graphql.fragment.GuideTemplate
import io.bosca.graphql.fragment.Metadata
import io.bosca.graphql.fragment.MetadataRelationship
import io.bosca.graphql.fragment.MetadataSupplementary
import io.bosca.graphql.fragment.Permission
import io.bosca.graphql.fragment.Source
import io.bosca.graphql.type.*
import io.bosca.util.toOptional
import java.io.File

class ContentMetadata(network: NetworkClient) : Api(network) {

    suspend fun getAll(offset: Int, limit: Int): List<Metadata> {
        val response = network.graphql.query(GetAllMetadataQuery(offset.toOptional(), limit.toOptional())).execute()
        response.validate()
        return response.data?.content?.findMetadata?.map { it.metadata } ?: emptyList()
    }

    suspend fun get(id: String): Metadata? {
        val response = network.graphql.query(GetMetadataQuery(id)).execute()
        response.validate()
        return response.data?.content?.metadata?.metadata
    }

    suspend fun getBySlug(slug: String): Metadata? {
        val response = network.graphql.query(GetSlugQuery(slug)).execute()
        response.validate()
        return response.data?.content?.slug?.onMetadata?.metadata
    }

    suspend fun getDocument(id: String, version: Int): Document? {
        val response = network.graphql.query(GetMetadataDocumentQuery(id, version)).execute()
        response.validate()
        return response.data?.content?.metadata?.document?.document
    }

    suspend fun getDocumentTemplate(id: String, version: Int): DocumentTemplate? {
        val response = network.graphql.query(GetMetadataDocumentTemplateQuery(id, version.toOptional())).execute()
        response.validate()
        return response.data?.content?.metadata?.documentTemplate?.documentTemplate
    }

    suspend fun getGuide(id: String, version: Int): Guide? {
        val response = network.graphql.query(GetMetadataGuideQuery(id, version)).execute()
        response.validate()
        return response.data?.content?.metadata?.guide?.guide
    }

    suspend fun getGuideTemplate(id: String, version: Int): GuideTemplate? {
        val response = network.graphql.query(GetMetadataGuideTemplateQuery(id, version)).execute()
        response.validate()
        return response.data?.content?.metadata?.guideTemplate?.guideTemplate
    }

    suspend fun getCategories(id: String, version: Int): List<Category>? {
        val response = network.graphql.query(GetMetadataCategoriesQuery(id, version.toOptional())).execute()
        response.validate()
        return response.data?.content?.metadata?.categories?.map { it.category }
    }

    suspend fun getSource(id: String): Source? {
        val response = network.graphql.query(GetSourceQuery(id)).execute()
        response.validate()
        return response.data?.content?.sources?.source?.source
    }

    suspend fun getParents(id: String): List<ParentCollection> {
        val response = network.graphql.query(GetMetadataParentsQuery(id)).execute()
        response.validate()
        return response.data?.content?.metadata?.parentCollections?.map { it.parentCollection } ?: emptyList()
    }

    suspend fun findMetadata(
        attributes: List<FindAttributeInput> = emptyList(),
        contentTypes: List<String>? = null,
        categoryIds: List<String>? = null,
        offset: Int = 0,
        limit: Int = 10,
        extensions: ExtensionFilterType? = null,
    ): List<Metadata> {
        val response = network.graphql.query(
            FindMetadataQuery(
                FindQueryInput(
                    attributes = listOf(FindAttributesInput(attributes)),
                    categoryIds = categoryIds.toOptional(),
                    contentTypes = contentTypes.toOptional(),
                    extensionFilter = extensions.toOptional(),
                    offset = offset.toOptional(),
                    limit = limit.toOptional()
                )
            )
        ).execute()
        response.validate()
        return response.data?.content?.findMetadata?.map { it.metadata } ?: emptyList()
    }

    suspend fun findMetadataBySystem(
        attributes: List<FindAttributeInput> = emptyList(),
        contentTypes: List<String>? = null,
        categoryIds: List<String>? = null,
        offset: Int = 0,
        limit: Int = 10,
        extensions: ExtensionFilterType? = null,
    ): List<Metadata> {
        val response = network.graphql.query(
            FindMetadataBySystemQuery(
                FindQueryInput(
                    attributes = listOf(FindAttributesInput(attributes)),
                    categoryIds = categoryIds.toOptional(),
                    contentTypes = contentTypes.toOptional(),
                    extensionFilter = extensions.toOptional(),
                    offset = offset.toOptional(),
                    limit = limit.toOptional()
                )
            )
        ).execute()
        response.validate()
        return response.data?.content?.findMetadataBySystem?.map { it.metadata } ?: emptyList()
    }

    suspend fun getCollectionTemplate(id: String): io.bosca.graphql.fragment.CollectionTemplate {
        val response = network.graphql.query(GetCollectionTemplateQuery(id)).execute()
        response.validate()
        return response.data?.content?.metadata?.collectionTemplate?.collectionTemplate ?: error("No collection template returned")
    }

    suspend fun getPermissions(id: String): List<Permission> {
        val response = network.graphql.query(GetMetadataPermissionsQuery(id)).execute()
        response.validate()
        return response.data?.content?.metadata?.permissions?.map { it.permission } ?: emptyList()
    }

    suspend fun getTextContents(id: String): String? {
        val response = network.graphql.query(GetTextContentsQuery(id)).execute()
        response.validate()
        return response.data?.content?.metadata?.content?.text
    }

    suspend fun getSupplementaryContentDownload(supplementaryId: String): MetadataSupplementaryContentDownload? {
        val response = network.graphql.query(GetMetadataSupplementaryDownloadQuery(supplementaryId)).execute()
        response.validate()
        return response.data?.content?.metadataSupplementary?.content?.metadataSupplementaryContentDownload
    }

    suspend fun getMetadataContentDownload(id: String): MetadataContentDownload? {
        val response = network.graphql.query(GetMetadataDownloadQuery(id)).execute()
        response.validate()
        return response.data?.content?.metadata?.content?.metadataContentDownload
    }

    suspend fun getMetadataContentUpload(id: String): MetadataContentUpload.Upload? {
        val response = network.graphql.query(GetMetadataUploadQuery(id)).execute()
        response.validate()
        return response.data?.content?.metadata?.content?.metadataContentUpload?.urls?.upload
    }

    suspend fun getSupplementaryTextContents(id: String, key: String): String? {
        val response = network.graphql.query(GetSupplementaryTextContentsQuery(id, key)).execute()
        response.validate()
        return response.data?.content?.metadata?.supplementary?.firstOrNull()?.content?.text
    }

    suspend fun add(metadata: MetadataInput): String? {
        val response = network.graphql.mutation(AddMetadataMutation(metadata)).execute()
        response.validate()
        return response.data?.content?.metadata?.add?.id
    }

    suspend fun addDocument(parentCollectionId: String, templateId: String, templateVersion: Int): Metadata {
        val response =
            network.graphql.mutation(AddDocumentMutation(parentCollectionId, templateId, templateVersion)).execute()
        response.validate()
        return response.data?.content?.metadata?.addDocument?.metadata ?: error("No metadata returned")
    }

    suspend fun addGuide(parentCollectionId: String, templateId: String, templateVersion: Int): Metadata {
        val response =
            network.graphql.mutation(AddGuideMutation(parentCollectionId, templateId, templateVersion)).execute()
        response.validate()
        return response.data?.content?.metadata?.addGuide?.metadata ?: error("No metadata returned")
    }

    suspend fun addSupplementary(supplementary: MetadataSupplementaryInput): MetadataSupplementary? {
        val response = network.graphql.mutation(AddMetadataSupplementaryMutation(supplementary)).execute()
        response.validate()
        return response.data?.content?.metadata?.addSupplementary?.metadataSupplementary
    }

    suspend fun edit(id: String, metadata: MetadataInput): String? {
        val response = network.graphql.mutation(EditMetadataMutation(id, metadata)).execute()
        response.validate()
        return response.data?.content?.metadata?.edit?.id
    }

    suspend fun setTextContent(id: String, contentType: String, content: String) {
        val response = network.graphql.mutation(SetMetadataTextContentsMutation(id, contentType, content)).execute()
        response.validate()
    }

    suspend fun setDocument(id: String, version: Int, document: DocumentInput) {
        val response = network.graphql.mutation(SetDocumentMutation(id, version, document)).execute()
        response.validate()
    }

    suspend fun setBible(id: String, version: Int, bible: BibleInput) {
        val response = network.graphql.mutation(SetBibleMutation(id, version, bible)).execute()
        response.validate()
    }

    suspend fun setSupplementaryTextContent(
        supplementaryId: String,
        contentType: String,
        content: String
    ) {
        val response = network.graphql.mutation(
            SetMetadataSupplementaryTextContentsMutation(
                supplementaryId,
                contentType,
                content
            )
        ).execute()
        response.validate()
    }

    suspend fun setSupplementaryTextContentWithMetadata(
        supplementaryId: String,
        contentType: String,
        content: String
    ) {
        val response = network.graphql.mutation(
            SetMetadataSupplementaryTextContentsMutation(
                supplementaryId,
                contentType,
                content
            )
        ).execute()
        response.validate()
    }

    suspend fun setJsonContent(id: String, contentType: String, content: Any) {
        val response = network.graphql.mutation(SetJsonContentsMutation(id, contentType, content)).execute()
        response.validate()
    }

    suspend fun delete(id: String): Boolean {
        val response = network.graphql.mutation(DeleteMetadataMutation(id)).execute()
        response.validate()
        return response.data?.content?.metadata?.delete ?: false
    }

    suspend fun deleteSupplementary(supplementaryId: String): Boolean {
        val response = network.graphql.mutation(DeleteSupplementaryMetadataMutation(supplementaryId)).execute()
        response.validate()
        return response.data?.content?.metadata?.deleteSupplementary ?: false
    }

    suspend fun setFileContents(id: String, file: Upload): Boolean {
        val response = network.graphql.mutation(SetContentsMutation(id, file.contentType.toOptional(), file)).execute()
        response.validate()
        return response.data?.content?.metadata?.setMetadataContents ?: false
    }

    suspend fun setSupplementaryContents(supplementaryId: String, file: Upload): Boolean {
        val response = network.graphql.mutation(
            SetMetadataSupplementaryContentsMutation(
                supplementaryId,
                file.contentType,
                file
            )
        ).execute()
        response.validate()
        return response.data?.content?.metadata?.setSupplementaryContents ?: false
    }

    suspend fun setSupplementaryContents(
        supplementaryId: String,
        file: File,
        contentType: String,
    ): Boolean = setSupplementaryContents(supplementaryId, file.toUpload(contentType))

    suspend fun setAttributes(id: String, attributes: Any?) {
        val response =
            network.graphql.mutation(SetMetadataAttributesMutation(id, attributes ?: emptyMap<Any, Any>())).execute()
        response.validate()
    }

    suspend fun mergeRelationshipAttributes(id1: String, id2: String, relationship: String, attributes: Any) {
        val response =
            network.graphql.mutation(MergeMetadataRelationshipAttributesMutation(id1, id2, relationship, attributes))
                .execute()
        response.validate()
    }

    suspend fun setSystemAttributes(id: String, attributes: Any?) {
        val response =
            network.graphql.mutation(SetMetadataSystemAttributesMutation(id, attributes ?: emptyMap<Any, Any>()))
                .execute()
        response.validate()
    }

    suspend fun getSupplementary(id: String): List<MetadataSupplementary> {
        val response = network.graphql.query(GetMetadataSupplementaryQuery(id)).execute()
        response.validate()
        return response.data?.content?.metadata?.supplementary?.map { it.metadataSupplementary } ?: emptyList()
    }

    suspend fun getRelationships(id: String): List<MetadataRelationship> {
        val response = network.graphql.query(GetMetadataRelationshipsQuery(id)).execute()
        response.validate()
        return response.data?.content?.metadata?.relationships?.map { it.metadataRelationship } ?: emptyList()
    }

    suspend fun getRelationshipsInverse(id: String): List<MetadataRelationship> {
        val response = network.graphql.query(GetMetadataRelationshipsInverseQuery(id)).execute()
        response.validate()
        return response.data?.content?.metadata?.relationships?.map { it.metadataRelationship } ?: emptyList()
    }

    suspend fun addRelationship(input: MetadataRelationshipInput) {
        val response = network.graphql.mutation(AddMetadataRelationshipMutation(input)).execute()
        response.validate()
    }

    suspend fun removeRelationship(id1: String, id2: String, relationship: String) {
        val response = network.graphql.mutation(RemoveMetadataRelationshipMutation(id1, id2, relationship)).execute()
        response.validate()
    }

    suspend fun addPermission(input: PermissionInput) {
        val response = network.graphql.mutation(AddMetadataPermissionMutation(input)).execute()
        response.validate()
    }

    suspend fun removePermission(input: PermissionInput) {
        val response = network.graphql.mutation(RemoveMetadataPermissionMutation(input)).execute()
        response.validate()
    }

    suspend fun setReady(id: String): Boolean {
        val response = network.graphql.mutation(SetMetadataReadyMutation(id)).execute()
        response.validate()
        return response.data?.content?.metadata?.setMetadataReady ?: false
    }

    suspend fun setPublic(id: String, public: Boolean) {
        val response = network.graphql.mutation(SetMetadataPublicMutation(id, public)).execute()
        response.validate()
    }

    suspend fun setPublicContent(id: String, public: Boolean) {
        val response = network.graphql.mutation(SetMetadataPublicContentMutation(id, public)).execute()
        response.validate()
    }

    suspend fun setPublicSupplementary(id: String, public: Boolean) {
        val response = network.graphql.mutation(SetMetadataPublicSupplementaryMutation(id, public)).execute()
        response.validate()
    }

    suspend fun deletePermanently(id: String) {
        val response = network.graphql.mutation(PermanentlyDeleteMetadataMutation(id)).execute()
        response.validate()
    }

    suspend fun addTrait(metadataId: String, traitId: String) {
        network.graphql.mutation(AddMetadataTraitMutation(metadataId, traitId)).execute().validate()
    }

    suspend fun removeTrait(metadataId: String, traitId: String) {
        network.graphql.mutation(RemoveMetadataTraitMutation(metadataId, traitId)).execute().validate()
    }
}