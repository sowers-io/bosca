package io.bosca.workflow.search

import com.meilisearch.sdk.Client
import com.meilisearch.sdk.Index
import com.meilisearch.sdk.model.TaskStatus
import kotlinx.coroutines.delay

class TaskFailedException(message: String) : Exception(message)
class TaskTimedOutException(message: String) : Exception(message)

suspend fun Index.suspendWaitForTask(id: Int) {
    var status: TaskStatus
    var tries = 0
    do {
        tries++
        if (tries >= 20) {
            throw TaskTimedOutException("Meilisearch task timed out")
        }
        delay(500)
        status = getTask(id).status
    } while ((status == TaskStatus.ENQUEUED) || (status == TaskStatus.PROCESSING))
    if (status == TaskStatus.FAILED) {
        throw TaskFailedException("Meilisearch task failed: ${getTask(id).error?.message}")
    }
}

suspend fun Client.suspendWaitForTask(id: Int) {
    var status: TaskStatus
    var tries = 0
    do {
        tries++
        if (tries >= 20) {
            throw TaskTimedOutException("Meilisearch task timed out")
        }
        delay(500)
        status = getTask(id).status
    } while ((status == TaskStatus.ENQUEUED) || (status == TaskStatus.PROCESSING))
    if (status == TaskStatus.FAILED) {
        throw TaskFailedException("Meilisearch task failed: ${getTask(id).error?.message}")
    }
}