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
