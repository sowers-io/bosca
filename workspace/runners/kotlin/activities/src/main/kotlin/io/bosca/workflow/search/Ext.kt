package io.bosca.workflow.search

import com.meilisearch.sdk.Client
import com.meilisearch.sdk.Index
import com.meilisearch.sdk.model.TaskStatus
import io.bosca.workflow.DelayedUntilException
import kotlinx.coroutines.delay
import java.time.ZonedDateTime

class TaskFailedException(message: String) : Exception(message)
class TaskTimedOutException() : DelayedUntilException(ZonedDateTime.now().plusMinutes(5))

suspend fun Index.suspendWaitForTask(id: Int) {
    var status: TaskStatus
    var tries = 0
    do {
        tries++
        if (tries >= 20) {
            throw TaskTimedOutException()
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
            throw TaskTimedOutException()
        }
        delay(500)
        status = getTask(id).status
    } while ((status == TaskStatus.ENQUEUED) || (status == TaskStatus.PROCESSING))
    if (status == TaskStatus.FAILED) {
        throw TaskFailedException("Meilisearch task failed: ${getTask(id).error?.message}")
    }
}