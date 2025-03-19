package io.bosca.workflow.search

import com.meilisearch.sdk.Client
import com.meilisearch.sdk.Index
import com.meilisearch.sdk.model.TaskStatus
import kotlinx.coroutines.delay

suspend fun Index.suspendWaitForTask(id: Int) {
    var status: TaskStatus
    do {
        delay(10)
        status = getTask(id).status
    } while ((status == TaskStatus.ENQUEUED) || (status == TaskStatus.PROCESSING))
    if (status == TaskStatus.FAILED) {
        throw Exception("Meilisearch task failed: ${getTask(id).error?.message}")
    }
}

suspend fun Client.suspendWaitForTask(id: Int) {
    var status: TaskStatus
    do {
        delay(10)
        status = getTask(id).status
    } while ((status == TaskStatus.ENQUEUED) || (status == TaskStatus.PROCESSING))
    if (status == TaskStatus.FAILED) {
        throw Exception("Meilisearch task failed: ${getTask(id).error?.message}")
    }
}