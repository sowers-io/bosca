package io.bosca.api

import io.bosca.graphql.SetCommentStatusMutation
import io.bosca.graphql.type.CommentStatus

class Comments(network: NetworkClient) : Api(network) {

    suspend fun updateStatus(commentId: Long, metadataId: String, metadataVersion: Int, status: String) {
        val commentStatus = when (status) {
            "PENDING" -> CommentStatus.PENDING
            "APPROVED" -> CommentStatus.APPROVED
            "PENDING_APPROVAL" -> CommentStatus.PENDING_APPROVAL
            else -> error("Invalid comment status: $status")
        }

        val response = network.graphql.mutation(
            SetCommentStatusMutation(
                metadataId = metadataId,
                metadataVersion = metadataVersion,
                commentId = commentId,
                status = commentStatus
            )
        ).execute()
        response.validate()
    }
}
