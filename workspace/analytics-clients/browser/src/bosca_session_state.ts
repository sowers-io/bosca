export class SessionState {
  private sessionTimeout: number
  private onSessionStart: () => void
  private isSessionActive: boolean
  private timeoutId: number | null = null

  constructor(sessionTimeout: number, onSessionStart: () => void) {
    this.sessionTimeout = sessionTimeout // Default timeout: 30 minutes
    this.onSessionStart = onSessionStart || (() => console.log('Session started'))

    this.isSessionActive = false
    this.timeoutId = null
    document.addEventListener('visibilitychange', this.handleVisibilityChange.bind(this))

    this.startSession()
  }

  onEvent() {
    this.resumeSession()
  }

  handleVisibilityChange() {
    if (document.visibilityState === 'hidden') {
      this.pauseSession()
    } else if (document.visibilityState === 'visible') {
      this.resumeSession()
    }
  }

  startSession() {
    if (!this.isSessionActive) {
      this.isSessionActive = true
      this.onSessionStart()
    }
    this.resetTimeout()
  }

  pauseSession() {
    if (this.isSessionActive) {
      this.clearTimeout()
    }
  }

  resumeSession() {
    if (!this.isSessionActive) {
      this.startSession()
    } else {
      this.resetTimeout()
    }
  }

  endSession() {
    if (this.isSessionActive) {
      this.isSessionActive = false
      this.clearTimeout()
    }
  }

  resetTimeout() {
    this.clearTimeout()
    // @ts-ignore
    this.timeoutId = setTimeout(() => {
      this.endSession()
    }, this.sessionTimeout)
  }

  clearTimeout() {
    if (this.timeoutId) {
      clearTimeout(this.timeoutId)
      this.timeoutId = null
    }
  }
}
