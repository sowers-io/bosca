package io.bosca.workflow

import java.time.ZonedDateTime

class DelayedUntilException(val delayedUntil: ZonedDateTime) : Exception() {
}