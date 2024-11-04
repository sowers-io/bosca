import { AnalyticElement, AnalyticEvent, AnalyticEventType, ContentElement, IAnalyticElement, IAnalyticEvent, IContentElement } from './event'


export interface AnalyticEventFactory {

  createEvent(event: IAnalyticEvent): Promise<AnalyticEvent>
}

let factory: AnalyticEventFactory = {
  async createEvent(event) {
    return new DefaultAnalyticEvent(event, new DefaultAnalyticElement(event.element, event.element ? event.element.extras : {}, event.element && event.element.content ? event.element.content.map((c) => new DefaultContentElement(c)) : []))
  },
}

export function getAnalyticEventFactory(): AnalyticEventFactory {
  return factory
}

export function setAnalyticEventFactory(newFactory: AnalyticEventFactory) {
  factory = newFactory
}

export class DefaultContentElement extends ContentElement {

  readonly id: string
  readonly type: string
  readonly index: number | undefined
  readonly percent: number | undefined

  constructor(element: IContentElement) {
    super()
    this.id = element.id
    this.type = element.type
    this.index = element.index
    this.percent = element.percent
  }

  clone(): ContentElement {
    return new DefaultContentElement({ id: this.id, type: this.type, index: this.index, percent: this.percent })
  }
}

export class DefaultAnalyticElement extends AnalyticElement {
  private readonly element: IAnalyticElement
  readonly extras: { [key: string]: string }
  readonly content: ContentElement[]

  constructor(element: IAnalyticElement, extras: { [key: string]: string }, content: ContentElement[]) {
    super()
    this.element = element
    this.extras = extras
    this.content = content
  }

  get id(): string {
    return this.element.id
  }

  get type(): string {
    return this.element.type
  }

  clone(): AnalyticElement {
    return new DefaultAnalyticElement(this.element, this.extras, this.content.map((c) => c.clone()))
  }
}

export class DefaultAnalyticEvent extends AnalyticEvent {

  private readonly event: IAnalyticEvent
  readonly element: AnalyticElement
  readonly created: Date = new Date()

  constructor(event: IAnalyticEvent, element: AnalyticElement) {
    super()
    this.event = event
    this.element = element
  }

  get type(): AnalyticEventType {
    return this.event.type
  }

  get name(): string {
    return this.event.type.toString()
  }

  toParameters(): any {
    const parameters: { [key: string ]: any } = {
      type: this.type.toString(),
      element_id: this.element.id,
      element_type: this.element.type,
      created: this.created.toISOString(),
    }
    if (this.element.extras) {
      for (const key in this.element.extras) {
        parameters['extra_' + key] = this.element.extras[key]
      }
    }
    if (this.element.content) {
      let ix = 0
      for (const content of this.element.content) {
        parameters['content_id_' + ix] = content.id
        parameters['content_id_type_' + ix] = content.type
        if (content.index !== undefined) {
          parameters['content_id_index_' + ix] = content.index

        }
        if (content.percent) {
          parameters['content_id_percent_' + ix] = content.percent
        }
        ix++
      }
    }
    return parameters
  }

  clone(): AnalyticEvent {
    return new DefaultAnalyticEvent(this.event, this.element.clone())
  }
}

