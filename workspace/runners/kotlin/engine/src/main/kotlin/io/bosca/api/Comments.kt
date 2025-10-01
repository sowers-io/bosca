package io.bosca.api

import io.bosca.graphql.SetCommentStatusMutation
import io.bosca.graphql.SetCommentSystemAttributesMutation
import io.bosca.graphql.MergeCommentSystemAttributesMutation
import io.bosca.graphql.type.CommentStatus

class Comments(network: NetworkClient) : Api(network) {

    suspend fun setStatus(metadataId: String, metadataVersion: Int, commentId: Long, status: CommentStatus) {
        val response = network.graphql.mutation(
            SetCommentStatusMutation(
                metadataId = metadataId,
                metadataVersion = metadataVersion,
                commentId = commentId.toInt(),
                status = status
            )
        ).execute()
        response.validate()
    }

    suspend fun setSystemAttributes(metadataId: String, version: Int, id: Int, attributes: Any?) {
        val response = network.graphql.mutation(
            SetCommentSystemAttributesMutation(
                metadataId,
                version,
                id,
                attributes ?: emptyMap<Any, Any>()
            )
        ).execute()
        response.validate()
    }

    suspend fun mergeSystemAttributes(metadataId: String, version: Int, id: Int, attributes: Any?) {
        val response = network.graphql.mutation(
            MergeCommentSystemAttributesMutation(
                metadataId,
                version,
                id,
                attributes ?: emptyMap<Any, Any>()
            )
        ).execute()
        response.validate()
    }
}
