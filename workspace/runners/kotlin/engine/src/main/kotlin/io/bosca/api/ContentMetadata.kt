package io.bosca.api

import com.apollographql.apollo.api.Optional
import com.apollographql.apollo.api.Upload
import io.bosca.graphql.*
import io.bosca.graphql.fragment.*
import io.bosca.graphql.fragment.Category
import io.bosca.graphql.fragment.Collection
import io.bosca.graphql.fragment.Document
import io.bosca.graphql.fragment.DocumentTemplate
import io.bosca.graphql.fragment.Guide
import io.bosca.graphql.fragment.Metadata
import io.bosca.graphql.fragment.MetadataContent
import io.bosca.graphql.fragment.MetadataRelationship
import io.bosca.graphql.fragment.MetadataSupplementary
import io.bosca.graphql.fragment.MetadataSupplementaryContent
import io.bosca.graphql.fragment.Permission
import io.bosca.graphql.fragment.Source
import io.bosca.graphql.type.*
import io.bosca.util.toOptional

class ContentMetadata(network: NetworkClient) : Api(network) {

    suspend fun get(id: String): Metadata? {
        val response = network.graphql.query(GetMetadataQuery(id)).execute()
        response.validate()
        return response.data?.content?.metadata?.metadata
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
        attributes: List<FindAttributeInput>,
        offset: Int,
        limit: Int,
        extensions: ExtensionFilterType? = null,
    ): List<Metadata> {
        val response = network.graphql.query(
            FindMetadataQuery(
                FindQueryInput(
                    attributes = listOf(FindAttributesInput(attributes)),
                    extensionFilter = extensions.toOptional(),
                    offset = offset.toOptional(),
                    limit = limit.toOptional()
                )
            )
        ).execute()
        response.validate()
        return response.data?.content?.findMetadata?.map { it.metadata } ?: emptyList()
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

    suspend fun getMetadataContent(id: String): MetadataContent? {
        val response = network.graphql.query(GetMetadataContentQuery(id)).execute()
        response.validate()
        return response.data?.content?.metadata?.content?.metadataContent
    }

    suspend fun getMetadataContentUpload(id: String): MetadataContentUpload.Upload? {
        val response = network.graphql.query(GetMetadataUploadQuery(id)).execute()
        response.validate()
        return response.data?.content?.metadata?.content?.metadataContentUpload?.urls?.upload
    }

    suspend fun getMetadataSupplementaryContent(id: String, key: String): MetadataSupplementaryContent? {
        val response = network.graphql.query(GetMetadataSupplementaryQuery(id)).execute()
        response.validate()
        return response.data?.content?.metadata?.supplementary?.firstOrNull { it.metadataSupplementary.key == key }?.metadataSupplementary?.content?.metadataSupplementaryContent
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
        val response = network.graphql.mutation(SetTextContentsMutation(id, contentType, content)).execute()
        response.validate()
    }

    suspend fun setDocument(id: String, version: Int, document: DocumentInput) {
        val response = network.graphql.mutation(SetDocumentMutation(id, version, document)).execute()
        response.validate()
    }

    suspend fun setSupplementaryTextContent(id: String, key: String, contentType: String, content: String) {
        val response =
            network.graphql.mutation(SetSupplementaryTextContentsMutation(id, key, contentType, content)).execute()
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

    suspend fun deleteSupplementary(id: String, key: String): Boolean {
        val response = network.graphql.mutation(DeleteSupplementaryMetadataMutation(id, key)).execute()
        response.validate()
        return response.data?.content?.metadata?.deleteSupplementary ?: false
    }

    suspend fun setFileContents(id: String, file: Upload): Boolean {
        val response =
            network.graphql.mutation(SetContentsMutation(id, Optional.presentIfNotNull(file.contentType), file))
                .execute()
        response.validate()
        return response.data?.content?.metadata?.setMetadataContents ?: false
    }

    suspend fun setSupplementaryContents(id: String, key: String, file: Upload): Boolean {
        val response =
            network.graphql.mutation(SetSupplementaryContentsMutation(id, key, file.contentType, file))
                .execute()
        response.validate()
        return response.data?.content?.metadata?.setSupplementaryContents ?: false
    }

    suspend fun setAttributes(id: String, attributes: Any?) {
        val response =
            network.graphql.mutation(SetMetadataAttributesMutation(id, attributes ?: emptyMap<Any, Any>())).execute()
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

    suspend fun setPublic(id: String, ready: Boolean) {
        val response = network.graphql.mutation(SetMetadataPublicMutation(id, ready)).execute()
        response.validate()
    }

    suspend fun setPublicContent(id: String, ready: Boolean) {
        val response = network.graphql.mutation(SetMetadataPublicContentMutation(id, ready)).execute()
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