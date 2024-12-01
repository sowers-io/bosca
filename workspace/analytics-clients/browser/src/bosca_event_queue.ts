import { Event } from './bosca_models'

export class PendingEvents {
  private readonly queue: EventQueue
  readonly eventCount: number
  readonly events: Event[] = []

  constructor(eventCount: number, queue: EventQueue, events: Event[]) {
    this.eventCount = eventCount
    this.queue = queue
    this.events = events
  }

  async commit() {
    return await this.queue.commit(this)
  }
    
  async rollback() {
    return await this.queue.rollback(this)
  }
}

export class EventQueue {

  private pending: Event[] = []
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

  async add(event: Event) {
    this.eventCount++
    this.pending.push(event)
    await this.queueStore()
  }

  async get(): Promise<PendingEvents | null> {
    if (!this.database || this.transaction) return null
    this.transaction = true
    const transaction = this.database!.transaction('events', 'readonly')
    const store = transaction.objectStore('events')
    const events: Event[] = await toResult(store.getAll())
    if (events.length === 0) return null
    return new PendingEvents(this.eventCount, this, events)
  }

  async commit(events: PendingEvents) {
    const transaction = this.database!.transaction('events', 'readwrite')
    const store = transaction.objectStore('events')
    for (const event of events.events) {
      store.delete(event.client_id)
    }
    transaction.commit()
    this.transaction = false
    return events.eventCount != this.eventCount
  }

  async rollback(events: PendingEvents) {
    this.transaction = false
    return events.eventCount != this.eventCount
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
      console.warn('database not ready')
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