import { AnalyticEvent } from './event'
import { AnalyticEventSink } from './sink'
import { ulid } from 'ulid'

export interface Context {
  app_id: string
  app_version: string
  client_id: string
  device: Device
  geo: Geo
  browser: Browser | null
  session_id: string
}

export interface Device {
  installation_id: string
  manufacturer: string
  model: string
  platform: string
  primary_locale: string
  system_name: string
  timezone: string
  type: string
  version: string
}

export interface Geo {
  city: string
  region: string
  country: string
}

export interface Browser {
  agent: string
}

export interface Event {
  client_id: string
  type: string
  created: number
  created_micros: number
  element: EventElement
}

export interface EventElement {
  id: string,
  type: string
  content: EventContent[]
  extras: { [key: string]: string }
}

export interface EventContent {
  id: string
  type: string
  index: number | null
  percent: number | null
}

export interface Events {
  context: Context
  sent: number
  sent_micros: number
  events: Event[]
}

export class BoscaSink extends AnalyticEventSink {

  readonly url: string
  readonly context: Context
  events: Events

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
    this.events = {
      context: this.context,
      events: [],
      sent: 0,
      sent_micros: 0,
    }
    const self = this
    setTimeout(() => {
      let _ = self.flush()
    }, 30_000)
  }

  protected async onAdd(_: AnalyticEvent, event: AnalyticEvent): Promise<void> {
    this.events.events.push({
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
    })
  }

  async flush() {
    if (this.events.context.device.installation_id === '') {
      this.events.context.device = await getDeviceInfo()
    }
    if (this.events.context.geo.country === '') {
      this.events.context.geo = await getGeo()
    }
    const events = this.events
    this.events = {
      context: this.context,
      events: [],
      sent: 0,
      sent_micros: 0,
    }
    events.sent = new Date().getTime()
    const response = await fetch(this.url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(events),
    })
    if (response.status != 200) {
      console.error('oops')
    }
  }
}

async function getDeviceInfo(): Promise<Device> {
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
    installation_id: await generateInstallationId(),
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

async function getGeo(): Promise<Geo> {
  return {} as Geo
}

async function generateInstallationId(): Promise<string> {
  const timestamp = Date.now().toString(36)
  const randomStr = Math.random().toString(36).substring(2, 8)
  return `${timestamp}-${randomStr}`
}