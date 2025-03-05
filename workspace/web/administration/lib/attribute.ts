import {
  AttributeType,
  AttributeUiType,
  type TemplateAttribute,
} from '~/lib/graphql/graphql.ts'
import type { BoscaClient } from '~/lib/bosca/client.ts'

export class AttributeState {
  readonly key: string
  readonly ui: AttributeUiType
  readonly type: AttributeType
  readonly list: boolean
  readonly name: string
  readonly description: string
  readonly configuration: any
  readonly hasWorkflows: boolean

  private _loading = false
  private _value: any
  private _invalidValue: boolean = false
  private _valueWarning: string | null = null

  constructor(
    key: string,
    ui: AttributeUiType,
    type: AttributeType,
    list: boolean,
    name: string,
    description: string,
    configuration: any,
    hasWorkflows: boolean = false,
  ) {
    this.key = key
    this.ui = ui
    this.type = type
    this.list = list
    this.name = name
    this.description = description
    this.configuration = configuration
    this.hasWorkflows = hasWorkflows
    this._value = null
  }

  get invalidValue(): boolean {
    return this._invalidValue
  }

  get valueWarning(): string | null {
    return this._valueWarning
  }

  get loading(): boolean {
    return this._loading
  }

  set loading(loading: boolean) {
    this._loading = loading
  }

  get dateTimeValue(): string {
    if (typeof this._value === 'number') {
      return new Date(this._value).toLocaleDateString('en-US', {
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
        hour12: true,
      }).replace(',', '')
    }
    return this._value?.toString() || ''
  }

  set dateTimeValue(value: string) {
    try {
      const date = new Date(Date.parse(value))
      if (isNaN(date.getTime())) throw new Error('invalid date')
      this._value = date.getTime()
      this._invalidValue = false
      if (this._value < new Date().getTime()) {
        this._valueWarning = 'Date is in the past.'
      } else {
        this._valueWarning = null
      }
    } catch (e) {
      this._invalidValue = true
      this._valueWarning = null
    }
    this._value = value
  }

  get value(): any {
    return this._value
  }

  set value(value: any) {
    if (typeof value === 'string' && value.length === 0) {
      this._value = undefined
    } else {
      this._value = value
    }
  }

  async setSupplementaryValue(
    client: BoscaClient<any>,
    metadataId: string,
    supplementaryKey: string,
  ) {
    switch (this.ui) {
      case AttributeUiType.Textarea:
      case AttributeUiType.Input: {
        this.value = await client.metadata.getSupplementaryText(
          metadataId,
          supplementaryKey,
        )
        if (this.value) {
          this.loading = false
        }
        break
      }
      case AttributeUiType.Collection: {
        const collections = await client.metadata.getSupplementaryJson(
          metadataId,
          supplementaryKey,
        )
        if (collections?.collections) {
          this.value = collections?.collections
          this.loading = false
        }
        break
      }
      default:
        this.loading = false
        break
    }
  }
}

export function newAttributeState(
  attribute: TemplateAttribute,
): AttributeState {
  return new AttributeState(
    attribute.key,
    attribute.ui,
    attribute.type,
    attribute.list,
    attribute.name,
    attribute.description,
    attribute.configuration,
    attribute.workflows && attribute.workflows.length > 0,
  )
}
