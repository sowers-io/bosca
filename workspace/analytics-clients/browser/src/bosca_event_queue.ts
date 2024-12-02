import { Context, Event } from './bosca_models'

export class PendingEvents {
  private readonly queue: EventQueue
  hasErrors = false
  readonly eventCount: number
  readonly events: PendingContextEvents[] = []
  constructor(eventCount: number, queue: EventQueue, events: PendingContextEvents[]) {
    this.eventCount = eventCount
    this.queue = queue
    this.events = events
  }

  async onError(_: any) {
    this.hasErrors = true
  }

  async finish(events: PendingContextEvents) {
    return await this.queue.finish(events)
  }

  async close() {
    return await this.queue.close(this)
  }
}

export class PendingContextEvents {
  readonly context: Context
  readonly events: Event[] = []

  constructor(context: Context, events: Event[]) {
    this.context = context
    this.events = events
  }
}

interface PendingEvent {
  client_id: string
  context: Context
  event: Event
}

export class EventQueue {

  private pending: PendingEvent[] = []
  private commitTimeout: any = null
  // eslint-disable-next-line no-undef
  private database: IDBDatabase | null = null
  private transaction = false
  private eventCount = 0

  constructor() {
    this.initialize().catch((e) => console.error('Database error:', e))
  }

  private async initialize() {
    // eslint-disable-next-line no-undef
    const request = indexedDB.open('EventDB', 1)
    request.onupgradeneeded = () => {
      const store = request.result
      if (!store.objectStoreNames.contains('events')) {
        store.createObjectStore('events', { keyPath: 'client_id' })
      }
    }
    this.database = await toResult(request)
  }

  async add(context: Context, event: Event) {
    this.eventCount++
    this.pending.push({ client_id: event.client_id, context: context, event: event })
    await this.queueStore()
  }

  async get(): Promise<PendingEvents | null> {
    if (!this.database || this.transaction) return null
    this.transaction = true
    const transaction = this.database!.transaction('events', 'readonly')
    const store = transaction.objectStore('events')
    const events: PendingEvent[] = await toResult(store.getAll())
    if (events.length === 0) return null
    const pendingEvents: PendingContextEvents[] = []
    let current: PendingContextEvents | null = null
    let currentSessionId: string | null = null
    for (const event of events) {
      if (currentSessionId !== event.context.session_id) {
        currentSessionId = event.context.session_id
        current = new PendingContextEvents(event.context, [])
        pendingEvents.push(current)
      }
      current!.events.push(event.event)
    }
    return new PendingEvents(this.eventCount, this, pendingEvents)
  }

  async finish(events: PendingContextEvents) {
    const transaction = this.database!.transaction('events', 'readwrite')
    const store = transaction.objectStore('events')
    for (const event of events.events) {
      store.delete(event.client_id)
    }
    transaction.commit()
  }

  async close(events: PendingEvents) {
    this.transaction = false
    return this.eventCount !== events.eventCount
  }

  private async queueStore() {
    if (this.commitTimeout) {
      clearTimeout(this.commitTimeout)
      this.commitTimeout = null
    }
    this.commitTimeout = setTimeout(async () => {
      await this.store()
    }, 500)
  }

  private async store() {
    if (!this.database) {
      await this.queueStore()
      return
    }
    if (this.pending.length === 0) return
    const transaction = this.database!.transaction('events', 'readwrite')
    const store = transaction.objectStore('events')
    const events = this.pending
    this.pending = []
    for (const pending of events) {
      store.add(pending)
    }
    transaction.commit()
  }
}

// eslint-disable-next-line no-undef
async function toResult<T>(request: IDBRequest<T>): Promise<T> {
  return await new Promise((resolve, reject) =>{
    request.onsuccess = () => {
      resolve(request.result)
    }
    request.onerror = () => {
      reject(request.error)
    }
  })
}