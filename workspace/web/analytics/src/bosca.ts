import { AnalyticEvent, AnalyticEventType } from './event'
import { AnalyticEventSink } from './sink'
import { ulid } from 'ulid'
import { EventQueue } from './bosca_event_queue'
import { Context, Device, EventContent, EventElement, Geo } from './bosca_models'
import { SessionState } from './bosca_session_state'
import { getAnalyticEventFactory } from './factory'

export class BoscaSink extends AnalyticEventSink {

  private readonly url: string
  private readonly context: Context
  private readonly queue = new EventQueue()
  private flushing = false
  private _flushed = 0
  private _failures = 0
  private timeout: any | null = null
  private sessionState: SessionState
  private retryDelay = 3_000

  constructor(url: string, appId: string, appVersion: string, clientId: string) {
    super()
    this.url = url
    this.context = {
      app_id: appId,
      app_version: appVersion,
      browser: typeof navigator === 'undefined' ? null : {
        // eslint-disable-next-line no-undef
        agent: navigator?.userAgent,
      },
      client_id: clientId,
      device: {
        installation_id: '',
        manufacturer: '',
        model: '',
        platform: '',
        primary_locale: '',
        system_name: '',
        timezone: '',
        type: '',
        version: '',
      },
      geo: {
        city: '',
        region: '',
        country: '',
      },
      session_id: ulid(),
    }
    const self = this
    this.sessionState = new SessionState(300000, async () => {
      self.context.session_id = ulid()
      const event = await getAnalyticEventFactory().createEvent({
        type: AnalyticEventType.session,
        element: {
          id: self.context.session_id,
          type: 'session',
          content: [],
          extras: {
            start: 'true',
          },
        },
      })
      try {
        await self.add(event)
      } catch (e) {
        console.error('failed to update session state: ', e)
      }
    })
  }

  get flushed() {
    return this._flushed
  }

  get failures() {
    return this._failures
  }

  async pendingSize(): Promise<number> {
    return this.queue.pendingSize()
  }

  async size(): Promise<number> {
    return this.queue.size()
  }

  protected async onAdd(_: AnalyticEvent, event: AnalyticEvent): Promise<void> {
    this.sessionState.onEvent().catch(e => console.error('failed to update session state: ', e))
    this.queue.add(this.context, {
      client_id: ulid(),
      type: event.type,
      created: event.created.getTime(),
      created_micros: 0,
      element: {
        id: event.element.id,
        type: event.element.type,
        content: event.element.content.map((c) => {
          return {
            id: c.id,
            type: c.type,
            index: c.index,
            percent: c.percent,
          } as EventContent
        }),
        extras: event.element.extras,
      } as EventElement,
    }).catch(e => console.error('failed to add event: ', e))
    this.queueFlush()
  }

  private queueRequests = 0

  private queueFlush() {
    if (this.timeout) {
      if (this.queueRequests > 10) {
        return
      }
      clearTimeout(this.timeout)
    }
    this.queueRequests++
    const self = this
    this.timeout = setTimeout(() => {
      self.timeout = null
      self.queueRequests = 0
      self.flush().catch(e => console.error('failed to flush: ', e))
    }, 1000)
  }

  async flush() {
    if (this.flushing) {
      console.log('already flushing...')
      this.queueFlush()
      return
    }
    this.flushing = true
    try {
      await this.initializeContext()
      const allPendingEvents = await this.queue.get()
      if (!allPendingEvents) return
      let errors = false
      try {
        for (const pendingEvents of allPendingEvents.events) {
          try {
            const events = {
              context: pendingEvents.context,
              events: pendingEvents.events,
              sent: new Date().getTime(),
              sent_micros: 0,
            }
            if (!events.context.device.installation_id) {
              events.context.device.installation_id = await this.generateInstallationId() || ''
            }
            const response = await fetch(this.url + '/events', {
              method: 'POST',
              headers: {
                'Content-Type': 'application/json',
              },
              body: JSON.stringify(events),
            })
            if (response.status != 200) {
              throw new Error('error sending events: ' + await response.text())
            }
            await allPendingEvents.finish(pendingEvents)
            this._flushed += pendingEvents.events.length
          } catch (e: any) {
            errors = true
            this._failures += pendingEvents.events.length
            console.error('failed to flush: ', e)
            await allPendingEvents.failed(pendingEvents)
          }
        }
      } finally {
        if (await allPendingEvents.close()) {
          this.queueFlush()
        } else if (errors) {
          this.retryDelay = Math.min(this.retryDelay * 2, 60_000)
          const self = this
          setTimeout(() => {
            self.flush().catch(e => console.error('failed to flush: ', e))
          }, this.retryDelay)
        }
      }
    } finally {
      this.flushing = false
    }
  }

  private async initializeContext() {
    if (this.context.device.installation_id === '') {
      this.context.device = await this.getDeviceInfo()
    }
    if (this.context.geo.country === '') {
      this.context.geo = await this.getGeo()
    }
  }

  private async getDeviceInfo(): Promise<Device> {
    const w = typeof window === 'undefined' ? {
      navigator: {
        userAgent: '',
        platform: '',
      },
    } :
    // eslint-disable-next-line no-undef
      window
    const userAgent = w.navigator.userAgent
    // eslint-disable-next-line no-undef
    const platform = w.navigator.platform

    // Parse user agent for device details
    const isIOS = /iPhone|iPad|iPod/.test(userAgent)
    const isAndroid = /Android/.test(userAgent)
    const isMobile = /Mobile/.test(userAgent)

    // Extract manufacturer and model
    let manufacturer = 'Unknown'
    let model = 'Unknown'

    if (isIOS) {
      manufacturer = 'Apple'
      if (userAgent.includes('iPhone')) model = 'iPhone'
      else if (userAgent.includes('iPad')) model = 'iPad'
      else if (userAgent.includes('iPod')) model = 'iPod'
    } else if (isAndroid) {
      manufacturer = userAgent.match(/Android.*?;.*?([^;]+)\s+Build/)?.[1]?.split(' ')[0] || 'Unknown'
      model = userAgent.match(/Android.*?;.*?([^;]+)\s+Build/)?.[1] || 'Unknown'
    } else {
      // For desktop, use platform info
      manufacturer = platform.split(' ')[0]
      model = platform
    }

    return {
      installation_id: await this.generateInstallationId(),
      manufacturer,
      model,
      platform: platform,
      // eslint-disable-next-line no-undef
      primary_locale: typeof navigator === 'undefined' ? 'zz' : navigator.language,
      system_name: isIOS ? 'iOS' : isAndroid ? 'Android' : 'Web',
      timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
      type: isMobile ? 'mobile' : 'desktop',
      version: userAgent.match(/(?:iPhone|CPU|Android|Edge|Chrome|Firefox|Safari|Opera|Version)[\s/:]\s?(\d+[._\d]*)/)?.[1] || 'Unknown',
    } as Device
  }

  private async getGeo(): Promise<Geo> {
    return {} as Geo
  }

  private async generateInstallationId(): Promise<string | null> {
    try {
      // eslint-disable-next-line no-undef
      let installationId = typeof localStorage === 'undefined' ? null : localStorage.getItem('__iid')
      if (!installationId) {
        const response = await fetch(this.url + '/register', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'Accept': 'application/json',
          },
        })
        if (response.ok) {
          const data = await response.json()
          installationId = data.id
          // eslint-disable-next-line no-undef
          if (typeof localStorage !== 'undefined') {
            // eslint-disable-next-line no-undef
            localStorage.setItem('__iid', installationId!)
          }
        } else {
          throw new Error(await response.json())
        }
      }
      return installationId
    } catch (e) {
      console.error('Failed to register IID:', e)
      return null
    }
  }
}
