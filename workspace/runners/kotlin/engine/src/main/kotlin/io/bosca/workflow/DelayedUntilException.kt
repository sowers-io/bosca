package io.bosca.workflow

import java.time.ZonedDateTime

open class DelayedUntilException(val delayedUntil: ZonedDateTime) : Exception() {

    override fun toString() = delayedUntil.toString()
}