export class SessionState {
  private sessionTimeout: number
  private onSessionStart: () => Promise<void>
  private isSessionActive: boolean
  private timeoutId: number | null = null

  constructor(sessionTimeout: number, onSessionStart: () => Promise<void>) {
    this.sessionTimeout = sessionTimeout // Default timeout: 30 minutes
    this.onSessionStart = onSessionStart || (() => console.log('Session started'))
    this.isSessionActive = false
    this.timeoutId = null
    if (typeof document !== 'undefined') {
      // eslint-disable-next-line no-undef
      document.addEventListener('visibilitychange', this.handleVisibilityChange.bind(this))
    }
    this.startSession()
  }

  async onEvent() {
    await this.resumeSession()
  }

  handleVisibilityChange() {
    // eslint-disable-next-line no-undef
    if (document.visibilityState === 'hidden') {
      this.pauseSession()
      // eslint-disable-next-line no-undef
    } else if (document.visibilityState === 'visible') {
      this.resumeSession()
    }
  }

  async startSession() {
    if (!this.isSessionActive) {
      this.isSessionActive = true
      await this.onSessionStart()
    }
    this.resetTimeout()
  }

  pauseSession() {
    if (this.isSessionActive) {
      this.clearTimeout()
    }
  }

  async resumeSession() {
    if (!this.isSessionActive) {
      await this.startSession()
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
