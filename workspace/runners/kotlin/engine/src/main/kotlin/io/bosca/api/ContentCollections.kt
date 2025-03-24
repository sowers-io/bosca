package io.bosca.api

import com.apollographql.apollo.api.Optional
import io.bosca.graphql.*
import io.bosca.graphql.fragment.*
import io.bosca.graphql.fragment.Collection
import io.bosca.graphql.fragment.CollectionSupplementary
import io.bosca.graphql.fragment.MetadataRelationship
import io.bosca.graphql.fragment.Permission
import io.bosca.graphql.type.*
import io.bosca.util.toOptional

sealed class CollectionItem {
    data class Collection(val collection: io.bosca.graphql.fragment.Collection) : CollectionItem()
    data class Metadata(val metadata: io.bosca.graphql.fragment.Metadata) : CollectionItem()

    val isCollection: Boolean
        get() = this is Collection

    val id: String
        get() = when (this) {
            is Collection -> collection.id
            is Metadata -> metadata.id
        }

    val name: String
        get() = when (this) {
            is Collection -> collection.name
            is Metadata -> metadata.name
        }
}

data class CollectionAndItems(
    val collection: Collection,
    val items: List<CollectionItem>
)

class ContentCollections(network: NetworkClient) : Api(network) {

    suspend fun list(id: String? = null): CollectionAndItems? {
        val response = network.graphql.query(GetCollectionListQuery(Optional.presentIfNotNull(id))).execute()
        response.validate()
        val list = response.data?.content?.collection?.collectionList ?: return null
        return CollectionAndItems(
            list.collection,
            list.items.mapNotNull {
                it.onCollection?.let { CollectionItem.Collection(it.collection) } ?:
                it.onMetadata?.let { CollectionItem.Metadata(it.metadata) }
            }
        )
    }

    suspend fun getAll(offset: Int, limit: Int): List<Collection> {
        val response = network.graphql.query(GetAllCollectionQuery(offset.toOptional(), limit.toOptional())).execute()
        response.validate()
        return response.data?.content?.findCollections?.map { it.collection } ?: emptyList()
    }

    suspend fun get(id: String? = null): Collection? {
        val response = network.graphql.query(GetCollectionQuery(Optional.presentIfNotNull(id))).execute()
        response.validate()
        return response.data?.content?.collection?.collection
    }

    suspend fun getParents(id: String): List<ParentCollection> {
        val response = network.graphql.query(GetCollectionParentsQuery(Optional.presentIfNotNull(id))).execute()
        response.validate()
        return response.data?.content?.collection?.collectionParents?.parentCollections?.map { it.parentCollection } ?: emptyList()
    }

    suspend fun getPermissions(id: String): List<Permission> {
        val response = network.graphql.query(GetCollectionPermissionsQuery(Optional.presentIfNotNull(id))).execute()
        response.validate()
        return response.data?.content?.collection?.permissions?.map { it.permission } ?: emptyList()
    }

    suspend fun getRelationships(id: String): List<CollectionRelationship> {
        val response = network.graphql.query(GetCollectionRelationshipsQuery(id)).execute()
        response.validate()
        return response.data?.content?.collection?.metadataRelationships?.map { it.collectionRelationship } ?: emptyList()
    }

    suspend fun add(collection: CollectionInput): String? {
        val response = network.graphql.mutation(AddCollectionMutation(collection)).execute()
        response.validate()
        return response.data?.content?.collection?.add?.id
    }

    suspend fun addSupplementary(supplementary: CollectionSupplementaryInput): CollectionSupplementary? {
        val response = network.graphql.mutation(AddCollectionSupplementaryMutation(supplementary)).execute()
        response.validate()
        return response.data?.content?.collection?.addSupplementary?.collectionSupplementary
    }

    suspend fun edit(id: String, collection: CollectionInput): String? {
        val response = network.graphql.mutation(EditCollectionMutation(id, collection)).execute()
        response.validate()
        return response.data?.content?.collection?.edit?.id
    }

    suspend fun setAttributes(id: String, attributes: Any?) {
        val response = network.graphql.mutation(SetCollectionAttributesMutation(id, attributes ?: emptyMap<Any, Any>())).execute()
        response.validate()
    }

    suspend fun addPermission(input: PermissionInput) {
        val response = network.graphql.mutation(AddCollectionPermissionMutation(input)).execute()
        response.validate()
    }

    suspend fun removePermission(input: PermissionInput) {
        val response = network.graphql.mutation(RemoveCollectionPermissionMutation(input)).execute()
        response.validate()
    }

    suspend fun setPublic(id: String, public: Boolean) {
        val response = network.graphql.mutation(SetCollectionPublicMutation(id, public)).execute()
        response.validate()
    }

    suspend fun setPublicList(id: String, ready: Boolean) {
        val response = network.graphql.mutation(SetCollectionPublicListMutation(id, ready)).execute()
        response.validate()
    }

    suspend fun setReady(id: String): Boolean {
        val response = network.graphql.mutation(SetCollectionReadyMutation(id)).execute()
        response.validate()
        return response.data?.content?.collection?.setReady ?: false
    }

    suspend fun delete(id: String): Boolean {
        val response = network.graphql.mutation(DeleteCollectionMutation(id)).execute()
        response.validate()
        return response.data?.content?.collection?.delete ?: false
    }

    suspend fun deletePermanently(id: String, recursive: Boolean = true) {
        val response = network.graphql.mutation(PermanentlyDeleteCollectionMutation(id, recursive.toOptional())).execute()
        response.validate()
    }

    suspend fun findCollections(attributes: List<FindAttributeInput>, offset: Int, limit: Int): List<Collection> {
        val response = network.graphql.query(FindCollectionsQuery(FindQueryInput(
            attributes = listOf(FindAttributesInput(attributes)),
            offset = offset.toOptional(),
            limit = limit.toOptional()
        ))).execute()
        response.validate()
        return response.data?.content?.findCollections?.map { it.collection } ?: emptyList()
    }

    suspend fun addCollection(collectionId: String, id: String): String? {
        val response = network.graphql.mutation(AddCollectionCollectionMutation(collectionId, id)).execute()
        response.validate()
        return response.data?.content?.collection?.addChildCollection?.id
    }

    suspend fun removeCollection(collectionId: String, id: String): String? {
        val response = network.graphql.mutation(RemoveCollectionCollectionMutation(collectionId, id)).execute()
        response.validate()
        return response.data?.content?.collection?.removeChildCollection?.id
    }

    suspend fun addMetadata(collectionId: String, metadataId: String): String? {
        val response = network.graphql.mutation(AddMetadataCollectionMutation(collectionId, metadataId)).execute()
        response.validate()
        return response.data?.content?.collection?.addChildMetadata?.id
    }

    suspend fun removeMetadata(collectionId: String, metadataId: String): String? {
        val response = network.graphql.mutation(RemoveMetadataCollectionMutation(collectionId, metadataId)).execute()
        response.validate()
        return response.data?.content?.collection?.removeChildMetadata?.id
    }

    suspend fun getSupplementaryContentDownload(supplementaryId: String): CollectionSupplementaryContentDownload? {
        val response = network.graphql.query(GetCollectionSupplementaryDownloadQuery(supplementaryId)).execute()
        response.validate()
        return response.data?.content?.collectionSupplementary?.content?.collectionSupplementaryContentDownload
    }

    suspend fun setSupplementaryTextContent(supplementaryId: String, contentType: String, content: String) {
        val response = network.graphql.mutation(SetCollectionSupplementaryTextContentsMutation(
            supplementaryId = supplementaryId,
            contentType = contentType,
            content = content
        )).execute()
        response.validate()
    }

    suspend fun deleteSupplementary(supplementaryId: String): Boolean {
        val response = network.graphql.mutation(DeleteSupplementaryCollectionMutation(supplementaryId)).execute()
        response.validate()
        return response.data?.content?.collection?.deleteSupplementary ?: false
    }
}