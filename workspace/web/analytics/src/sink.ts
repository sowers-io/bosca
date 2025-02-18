import { AnalyticEvent, AnalyticEventType, IAnalyticElement, IAnalyticEvent } from './event'
import { getAnalyticEventFactory } from './factory'

let sinks: AnalyticEventSink[] = []

export function addSink(sink: AnalyticEventSink) {
  sinks.push(sink)
}

export interface AnalyticEventInterceptor {

  intercept(event: AnalyticEvent): Promise<AnalyticEvent>
}

export abstract class AnalyticEventSink {

  private interceptors: AnalyticEventInterceptor[] = []

  addInterceptor(interceptor: AnalyticEventInterceptor) {
    this.interceptors.push(interceptor)
  }

  protected abstract onAdd(original: AnalyticEvent, event: AnalyticEvent): Promise<void>

  async add(event: AnalyticEvent): Promise<void> {
    const original = event
    for (const interceptor of this.interceptors) {
      event = await interceptor.intercept(event.clone())
    }
    await this.onAdd(original, event)
  }
}

export async function logEvent(event: IAnalyticEvent): Promise<void> {
  const ev = await getAnalyticEventFactory().createEvent(event)
  const promises = []
  for (const sink of sinks) {
    promises.push(sink.add(ev))
  }
  await Promise.all(promises)
}

export async function logImpression(element: IAnalyticElement): Promise<void> {
  await logEvent({
    type: AnalyticEventType.impression,
    element: element,
  })
}

export async function logInteraction(element: IAnalyticElement): Promise<void> {
  await logEvent({
    type: AnalyticEventType.interaction,
    element: element,
  })
}

export async function logCompletion(element: IAnalyticElement): Promise<void> {
  await logEvent({
    type: AnalyticEventType.completion,
    element: element,
  })
}