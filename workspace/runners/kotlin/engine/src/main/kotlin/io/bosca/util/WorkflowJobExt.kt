package io.bosca.util

import io.bosca.graphql.fragment.WorkflowActivityParameter
import io.bosca.graphql.fragment.WorkflowJob

fun WorkflowJob.getMetadataSupplementary(parameter: WorkflowActivityParameter) =
    metadata?.metadata?.supplementary?.firstOrNull {
        it.metadataSupplementary.key == parameter.value && (it.metadataSupplementary.planId == null || it.metadataSupplementary.planId == planId.id || it.metadataSupplementary.planId == this.parent?.id)
    }?.metadataSupplementary


fun WorkflowJob.getCollectionSupplementary(parameter: WorkflowActivityParameter) =
    collection?.collection?.supplementary?.firstOrNull {
        it.collectionSupplementary.key == parameter.value && (it.collectionSupplementary.planId == planId.id || it.collectionSupplementary.planId == null || it.collectionSupplementary.planId == this.parent?.id)
    }?.collectionSupplementary ?: profile?.profile?.collection?.collection?.supplementary?.firstOrNull {
        it.collectionSupplementary.key == parameter.value && (it.collectionSupplementary.planId == planId.id || it.collectionSupplementary.planId == null || it.collectionSupplementary.planId == this.parent?.id)
    }?.collectionSupplementary